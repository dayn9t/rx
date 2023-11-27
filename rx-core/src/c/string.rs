use std::ffi::{c_char, CString};
use std::path::Path;

/// 字符串表装器
pub struct StrWrapper {
    c_string: CString,
}

impl StrWrapper {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        Self {
            c_string: CString::new(s).unwrap(),
        }
    }

    pub fn from<P>(p: &P) -> Self
    where
        P: AsRef<Path>,
    {
        let s = crate::fs::to_str(p).to_owned();
        Self::new(s)
    }

    /// 获取C字符串指针
    pub fn as_cstr(&self) -> *const c_char {
        self.c_string.as_ptr() as *const c_char
    }
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
