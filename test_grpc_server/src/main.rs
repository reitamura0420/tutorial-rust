use byteorder::{BigEndian, ByteOrder};
use rustc_hash::FxHashMap;
use solicit::http::connection::HttpFrame;
use solicit::http::frame::{
    unpack_header, DataFrame, Frame, FrameIR, HeadersFlag, HeadersFrame, HttpSetting, PingFrame,
    RawFrame, SettingsFrame, WindowUpdateFrame, FRAME_HEADER_LEN,
};
use solicit::http::{Header, INITIAL_CONNECTION_WINDOW_SIZE};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::io::{Cursor, Read, Write};
use std::net::TcpListener;
use std::os::unix::io::AsRawFd;
use std::time::SystemTime;
use std::{env, io, thread};
use thiserror::Error;

#[macro_use]
extern crate slice_as_array;

#[derive(Error, Debug)]
pub enum Error {
    #[error("wrong preface string")]
    WrongPreface,
    #[error("wrong headers")]
    WrongHeaders,
    #[error("wrong http frame")]
    WrongHttpFrame(solicit::http::HttpError),
}

use prost::Message;

pub mod items {
    include!(concat!(env!("OUT_DIR"), "/helloworld.rs"));
}

pub fn create_hello(name: String) -> items::HelloRequest {
    let mut hello = items::HelloRequest::default();
    hello.name = name;
    hello
}

pub fn serialize_hello(hello: &items::HelloRequest) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(hello.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    hello.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_hello(buf: &Vec<u8>) -> Result<items::HelloReply, prost::DecodeError> {
    items::HelloReply::decode(Cursor::new(buf))
}

struct Client {
    stream: TcpStream,
    buffer: bytes::BytesMut,
    established: bool,
    wqueue: VecDeque<Vec<u8>>,
    ping: u64,
    window_size: i32,
    decoder: hpack::Decoder<'static>,
}

impl Client {
    fn new(mut stream: TcpStream) -> Self {
        // let raw_fd = stream.as_raw_fd();

        // let flags =
        //     nix::fcntl::OFlag::from_bits(nix::fcntl::fcntl(raw_fd, nix::fcntl::F_GETFL).unwrap())
        //         .unwrap()
        //         | nix::fcntl::OFlag::O_NONBLOCK;
        // nix::fcntl::fcntl(raw_fd, nix::fcntl::F_SETFL(flags)).unwrap();

        let mut c = Client {
            stream,
            buffer: bytes::BytesMut::new(),
            established: false,
            wqueue: VecDeque::new(),
            ping: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            window_size: INITIAL_CONNECTION_WINDOW_SIZE,
            decoder: hpack::Decoder::new(),
        };
        let mut s = SettingsFrame::new();
        s.add_setting(HttpSetting::MaxFrameSize(16384));
        c.queue(s);
        c
    }

    fn queue<T: FrameIR>(&mut self, frame: T) {
        let mut buf = Cursor::new(Vec::new());
        frame.serialize_into(&mut buf).unwrap();
        self.wqueue.push_back(buf.into_inner());
    }

    fn handle_dataframe(&mut self, frame: DataFrame) {
        let mut buf = Cursor::new(Vec::new());

        let request_str = String::from("blue");
        let req = create_hello(request_str);
        let request_vector = serialize_hello(&req);
        let mut request = Cow::Borrowed(&request_vector);

        let stream_id = frame.get_stream_id();

        WindowUpdateFrame::for_connection(frame.payload_len())
            .serialize_into(&mut buf)
            .unwrap();
        PingFrame::with_data(self.ping)
            .serialize_into(&mut buf)
            .unwrap();

        self.ping += 1;

        let http_status_headers: Vec<u8> = {
            let headers = vec![
                Header::new(b":status", b"200"),
                Header::new(b"content-type".to_vec(), b"application/grpc".to_vec()),
            ];
            hpack::Encoder::new().encode(headers.iter().map(|h| (h.name(), h.value())))
        };
        let grpc_status_headers: Vec<u8> = {
            let headers = vec![
                Header::new(b"grpc-status".to_vec(), b"0"),
                Header::new(b"grpc-message".to_vec(), b"".to_vec()),
            ];
            hpack::Encoder::new().encode(headers.iter().map(|h| (h.name(), h.value())))
        };

        let mut frame = HeadersFrame::new(http_status_headers.to_vec(), stream_id);
        frame.set_flag(HeadersFlag::EndHeaders);
        frame.serialize_into(&mut buf).unwrap();

        let reply = self.say_hello(&req);

        println!("{:?}", request_vector);

        let frame = DataFrame::with_data(stream_id, request_vector);
        self.window_size -= frame.payload_len() as i32;
        frame.serialize_into(&mut buf).unwrap();

        let mut frame = HeadersFrame::new(grpc_status_headers.to_vec(), stream_id);
        frame.set_flag(HeadersFlag::EndHeaders);
        frame.set_flag(HeadersFlag::EndStream);
        frame.serialize_into(&mut buf).unwrap();

        println!("{:?}", buf);

        self.wqueue.push_back(buf.into_inner());
    }

    fn flush(&mut self) {
        while let Some(buf) = self.wqueue.pop_front() {
            if buf.len() > self.window_size as usize {
                self.wqueue.push_front(buf);
                return;
            }
            println!("write");
            match self.stream.write(&buf) {
                Ok(_) => {}
                Err(_) => {
                    self.wqueue.push_front(buf);
                    return;
                }
            }
        }
    }

    fn handle(&mut self) -> Result<(), Error> {
        const RESERVE: usize = 8192;

        let len = self.buffer.len();
        self.buffer.reserve(len + RESERVE);
        unsafe {
            self.buffer.set_len(len + RESERVE);
        }
        self.stream.read(&mut self.buffer.as_mut()[len..]);

        if !self.established {
            match self.consume(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n".to_vec().len()) {
                Some(buf) => {
                    self.established = true;
                }
                None => {
                    return Ok(());
                }
            }
        }

        if self.established {
            loop {
                println!("{}", self.buffer.len());
                println!("{}", FRAME_HEADER_LEN);
                if self.buffer.len() < FRAME_HEADER_LEN {
                    break;
                }
                let header = unpack_header(
                    slice_as_array!(
                        &self.buffer.as_ref()[0..FRAME_HEADER_LEN],
                        [u8; FRAME_HEADER_LEN]
                    )
                    .unwrap(),
                );

                match self.consume(FRAME_HEADER_LEN + header.0 as usize) {
                    Some(buf) => {
                        let raw = RawFrame::from(buf);
                        match HttpFrame::from_raw(&raw) {
                            Ok(frame) => match frame {
                                HttpFrame::DataFrame(frame) => {
                                    self.handle_dataframe(frame);
                                }
                                HttpFrame::HeadersFrame(frame) => {
                                    let REQUEST_HEADERS: HashMap<Vec<u8>, Vec<u8>> = vec![(
                                        String::from(":path").into_bytes(),
                                        String::from("/helloworld.Greeter/SayHello").into_bytes(),
                                    )]
                                    .into_iter()
                                    .collect();
                                    for (k, v) in
                                        self.decoder.decode(&frame.header_fragment()).unwrap()
                                    {
                                        if let Some(expected) = REQUEST_HEADERS.get(&k) {
                                            if &v != expected {
                                                // should send an error response instead
                                                return Err(Error::WrongHeaders);
                                            }
                                        }
                                    }
                                }
                                HttpFrame::RstStreamFrame(_) => {
                                    return Err(Error::WrongHeaders);
                                }
                                HttpFrame::SettingsFrame(frame) => {
                                    println!("{:?}", frame);
                                    if !frame.is_ack() {
                                        self.queue(SettingsFrame::new_ack());
                                    }
                                }
                                HttpFrame::PingFrame(frame) => {
                                    println!("{:?}", frame);
                                    if !frame.is_ack() {
                                        self.queue(PingFrame::new_ack(frame.opaque_data()));
                                    }
                                }
                                HttpFrame::GoawayFrame(_) => {}
                                HttpFrame::WindowUpdateFrame(frame) => {
                                    self.window_size += frame.increment() as i32;
                                }
                                HttpFrame::UnknownFrame(_) => {}
                            },
                            Err(e) => return Err(Error::WrongHttpFrame(e)),
                        }
                    }
                    None => break,
                }
            }
        }
        println!("nuketa");
        self.flush();
        Ok(())
    }

    fn consume(&mut self, size: usize) -> Option<Vec<u8>> {
        if self.buffer.len() < size {
            return None;
        }
        Some(self.buffer.split_to(size).to_vec())
    }

    fn say_hello(&self, req: &items::HelloRequest) -> items::HelloReply {
        let message = items::HelloReply {
            message: format!("Hello {}", req.name),
        };
        println!("{:?}", message);
        message
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:50051").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            let mut client = Client::new(stream);
            client.handle()
        });
    }
}
