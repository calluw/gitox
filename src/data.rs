use sha1::{Digest, Sha1};
use std::fs;

const GIT_DIR: &str = ".gitox";
const OBJECT_DIR: &str = ".gitox/objects";

#[derive(Debug, PartialEq)]
pub enum ObjectType {
    Blob,
}

#[derive(Debug)]
pub struct Object {
    pub t: ObjectType,
    pub contents: Vec<u8>,
}

fn get_type_from_bytes(bytes: &[u8]) -> Option<ObjectType> {
    match bytes {
        b"blob" => Some(ObjectType::Blob),
        _ => None,
    }
}

pub fn init() -> std::io::Result<()> {
    fs::create_dir_all(GIT_DIR)?;
    fs::create_dir_all(OBJECT_DIR)?;
    Ok(())
}

pub fn hash_object(contents: &[u8], t: ObjectType) -> std::io::Result<String> {
    let t_str = match t {
        ObjectType::Blob => "blob",
    };

    // Format of an object is its type, null byte then the contents
    let data = [t_str.as_bytes(), b"\x00", contents].concat();
    let hash = Sha1::digest(&data);
    let oid = format!("{:x}", hash);

    fs::write(format!("{}/{oid}", OBJECT_DIR, oid = oid), data)?;
    Ok(oid)
}

pub fn get_object(oid: &str, expected: Option<ObjectType>) -> std::io::Result<Object> {
    let raw = fs::read(format!("{}/{oid}", OBJECT_DIR, oid = oid))?;

    // Object type is the first byte slice before a null byte
    let null_index = raw.iter().position(|byte| *byte == 0).unwrap();
    let (t_bytes, contents) = raw.split_at(null_index);
    let t = get_type_from_bytes(t_bytes).unwrap();

    if let Some(expected) = expected {
        assert_eq!(t, expected);
    }

    Ok(Object {
        t: t,
        contents: contents.to_vec(),
    })
}
