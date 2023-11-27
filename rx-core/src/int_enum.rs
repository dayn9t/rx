//use std::fmt;

//use super::types::*;

/// 整数枚举
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default)]
pub struct IntEnum(i32);

//TODO: 整数枚举
/*
impl IntEnum {
    /// 创建时间戳
    pub fn new(secs: i32) -> Self {
        IntEnum(secs)
    }

    /// 从字符串解析
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<IntEnum> {
        let dt = NaiveDateTime::parse_from_str(s, fmt)?;
        Ok(IntEnum::from(dt))
    }

}


impl From<NaiveDateTime> for IntEnum {
    fn from(dt: NaiveDateTime) -> Self {
        IntEnum(dt.timestamp() as i32)
    }
}

impl Into<NaiveDateTime> for IntEnum {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0 as i64, 0)
    }
}

impl fmt::Display for IntEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_datetime().to_string())
    }
}

impl Serialize for IntEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt: NaiveDateTime = self.clone().into();
        dt.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for IntEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dt = NaiveDateTime::deserialize(deserializer);
        dt.map(|v| IntEnum::from(v))
    }
}

/// 日期时间转换成时间戳
pub fn timestamp_or(time: &Option<NaiveDateTime>, v: IntEnum) -> IntEnum {
    if let Some(t) = time {
        IntEnum::from(t.to_owned())
    } else {
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let s = "2000-01-01 00:00:00";
        let t = IntEnum::parse_from_common_str(s).unwrap();
        assert_eq!(t.to_string(), s);
        println!("{}", t);
    }
}
*/
#[cfg(test)]
mod tests {
    use crate::serde_export::*;

    use crate::text::json;
    use std::mem::size_of;

    /// 整数枚举
    #[derive(Debug, Deserialize, Serialize)]
    enum Test {
        A(i32),
        B(i32),
    }

    enum TestInt {
        I1 = 1,
    }

    #[test]
    fn it_works() {
        //let a = Test::A(9);
        let a = Test::B(11);
        let _i = TestInt::I1 as i32;
        let _s = json::to_pretty(&a).unwrap();

        let _size = size_of::<TestInt>();
        //assert_eq!(_size, 4); =0?

        //assert_eq!(s, "4");
    }
}
