use std::fmt::{self, Display, LowerHex};
use std::marker::PhantomData;
use std::usize;

///强类型Id
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Id<C, T = usize>(T, PhantomData<C>);

impl<C, T> Id<C, T> {
    /// 建立ID
    pub fn new(n: T) -> Id<C, T> {
        Id::<C, T>(n, PhantomData::<C>)
    }
}
/*
解析可能失败，用TryFrom？
impl<C> From<&str> for Id<C, usize> {
    ///格式化显示ID
    fn from(s: &str) -> Self
    {
        let n = usize::from_str_radix(s, 16);
        Self::new(n)
    }
}*/

impl<C, T: LowerHex> Display for Id<C, T> {
    ///格式化显示ID
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:x}", self.0)
    }
}

#[test]
fn it_equ() {
    type IId = Id<i32>;
    let i1 = IId::new(1);
    let i2 = IId::new(1);
    assert_eq!(i1, i2);
}
