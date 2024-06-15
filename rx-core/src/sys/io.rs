use std::fs::File;
use std::io::{copy, Read};
use std::path::Path;

pub fn save_bytes<R: ?Sized>(reader: &mut R, path: impl AsRef<Path>)
where
    R: Read,
{
    let mut dest = File::create(path.as_ref()).unwrap();

    copy(reader, &mut dest).unwrap();
}
/*
pub fn copy<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> std::io::Result<u64>
    where
        R: Read,
        W: Write,
        */
