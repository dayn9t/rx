use deref_derive::{Deref, DerefMut};
use std::ffi::{c_char, CString};
use std::path::Path;

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
}
