use deref_derive::{Deref, DerefMut};
use std::ffi::{c_char, CString};
use std::io::ErrorKind;
use std::path::Path;
use std::{io, ptr};

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

/*
impl Deref for StringAdapter {
    type Target = *const c_char;

    fn deref(&self) -> &Target {
        self.as_cstr()
    }
}*/

/*
fn main1() {
    let c_buf: *const c_char = unsafe { hello() };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary
}
*/
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

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
}
