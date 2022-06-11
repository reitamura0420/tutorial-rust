use std::io::Cursor;

use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod items {
    include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
}

pub fn create_large_shirt(color: String) -> items::Shirt {
    let mut shirt = items::Shirt::default();
    shirt.color = color;
    shirt.set_size(items::shirt::Size::Large);
    shirt
}

pub fn serialize_shirt(shirt: &items::Shirt) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize_shirt(buf: &Vec<u8>) -> Result<items::Shirt, prost::DecodeError> {
    items::Shirt::decode(Cursor::new(buf))
}

fn main() -> Result<(), prost::DecodeError> {
    let request = String::from("blue");

    let shift_request = create_large_shirt(request);
    let request_vector = serialize_shirt(&shift_request);

    let request_deserialized_result = match deserialize_shirt(&request_vector) {
        Ok(request_deserialized_result) => request_deserialized_result,
        Err(e) => return Err(e),
    };
    println!("{:#?}", request_deserialized_result);
    Ok(())
}
