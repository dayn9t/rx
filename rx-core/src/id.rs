use std::fmt::{self, Debug, Display, LowerHex};
use std::hash::Hash;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

//use serde::de::DeserializeOwned;
//trait IdType: Serialize + DeserializeOwned + Copy + Clone + Eq + PartialEq + Hash + Display + Debug {}

//TODO: 捏造数值类型

///强类型Id
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Id<C, T = usize>(T, PhantomData<C>);

impl<C, T> Id<C, T>
where
    T: LowerHex,
{
    /// 建立ID
    pub fn new(n: T) -> Id<C, T> {
        Id::<C, T>(n, PhantomData::<C>)
    }

    /// 获取16进制字符串表示
    pub fn to_hex_string(&self) -> String {
        format!("#{:x}", self.0)
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

impl<C, T: LowerHex> Display for Id<C, T>
where
    T: Display,
{
    ///格式化显示ID
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn it_equ() {
    use std::mem::size_of;

    type IId1 = Id<f64>;
    assert_eq!(size_of::<IId1>(), 8);

    let i1 = IId1::new(1);
    assert_eq!(i1, i1);

    let i1 = IId1::new(17);
    assert_eq!(i1.to_string(), "17");

    type IId2 = Id<f32, i32>;
    assert_eq!(size_of::<IId2>(), 4);

    let i2 = IId2::new(1);
    assert_eq!(i2, i2);

    //assert_eq!(i1, i2);
}
