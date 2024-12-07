use core::str::Utf8Error;
use std::ffi::{CStr, CString, c_char};
use std::io::ErrorKind;
use std::path::Path;
use std::{io, ptr};

use derive_more::{Deref, DerefMut};

/// CString 增强版本
#[derive(Debug, Default, Deref, DerefMut)]
pub struct CStringX(CString);

impl CStringX {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        Self(CString::new(s).unwrap())
    }

    pub fn from<P>(p: &P) -> Self
    where
        P: AsRef<Path>,
    {
        let s = crate::sys::fs::to_str(p).to_owned();
        Self::new(s)
    }

    /// 获取C字符串指针
    pub fn ptr(&self) -> *const c_char {
        self.0.as_ptr() as *const c_char
    }
}

/// 构造 C 字符串
pub fn cstr(s: impl Into<Vec<u8>>) -> CStringX {
    CStringX::new(s)
}

/// 字接数组 复制到 C字符数组
pub fn bytes_fill_chars<const N: usize>(src: &[u8], dst: &mut [c_char; N]) -> io::Result<()> {
    let len = src.len().min(N - 1);

    if src.len() > N - 1 {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "buffer length is insufficient",
        ));
    }

    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr() as *mut u8, len);
    }
    dst[len] = 0;
    Ok(())
}

/// 字符串 转换为 C字符数组
pub fn str_to_chars<const N: usize>(s: &str) -> io::Result<[c_char; N]> {
    let mut c_array: [c_char; N] = [0; N];
    bytes_fill_chars(s.as_bytes(), &mut c_array)?;
    Ok(c_array)
}

/// C字符数组转换为字符串
pub fn chars_to_str<const N: usize>(s: [c_char; N]) -> Result<String, Utf8Error> {
    let c_str = unsafe { CStr::from_ptr(s.as_ptr()) };
    c_str.to_str().map(|s| s.to_owned())
}

#[cfg(test)]
mod tests {
    use std::ffi::CStr;

    use super::*;

    #[test]
    fn test_new() {
        let s = "hello";
        let wrapper = CStringX::new(s);
        let c_str = unsafe { CStr::from_ptr(wrapper.ptr()) };
        assert_eq!(c_str.to_str().unwrap(), s);
    }

    #[test]
    fn test_from() {
        let path = std::path::Path::new("test_path");
        let wrapper = CStringX::from(&path);
        let expected_str = crate::sys::fs::to_str(&path).to_owned();
        let c_str = unsafe { CStr::from_ptr(wrapper.ptr()) };
        assert_eq!(c_str.to_str().unwrap(), expected_str);
    }

    #[test]
    fn test_as_cstr() {
        let s = "test";
        let wrapper = CStringX::new(s);
        let c_str = unsafe { CStr::from_ptr(wrapper.ptr()) };
        assert_eq!(c_str.to_str().unwrap(), s);
    }

    #[test]
    fn test_bytes_fill_c_chars_success() {
        let src = b"hello";
        let mut dst = [0 as c_char; 6]; // 5 characters + null terminator
        bytes_fill_chars(src, &mut dst).expect("Failed to fill c_chars");
        let c_str = unsafe { CStr::from_ptr(dst.as_ptr()) };
        assert_eq!(c_str.to_str().unwrap(), "hello");
    }

    #[test]
    fn test_bytes_fill_c_chars_insufficient_buffer() {
        let src = b"hello";
        let mut dst = [0 as c_char; 5]; // Insufficient buffer
        let result = bytes_fill_chars(src, &mut dst);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn test_chars_to_str_success() {
        let src = b"hello\0";
        let mut c_array = [0 as c_char; 6];
        for (i, &byte) in src.iter().enumerate() {
            c_array[i] = byte as c_char;
        }
        let result = chars_to_str(c_array);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "hello");
    }

    #[test]
    fn test_chars_to_str_invalid_utf8() {
        let c = 0x80u8 as i8 as c_char;
        let src = [c, c, c, c, c, 0];
        let result = chars_to_str(src);
        assert!(result.is_err());
    }
}
