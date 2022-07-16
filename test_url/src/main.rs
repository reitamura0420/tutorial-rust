extern crate base64;
extern crate percent_encoding;
extern crate url;

use base64::{decode, encode};
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use std::str::{from_utf8, Utf8Error};
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    // percent();
    url_encode();
    base_encode();
}

fn percent() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, NON_ALPHANUMERIC);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}

fn url_encode() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}

fn base_encode() {
    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded).unwrap();

    println!("origin: {}", from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", from_utf8(&decoded).unwrap());
}
