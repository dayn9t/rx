use std::fs::File;
use std::io::{Read, copy};
use std::path::Path;

pub fn save_bytes<R: Read>(reader: &mut R, path: impl AsRef<Path>) {
    let mut dest = File::create(path.as_ref()).unwrap();

    copy(reader, &mut dest).unwrap();
}
/*
pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> std::io::Result<u64>
    where
        R: Read,
        W: Write,
        */
