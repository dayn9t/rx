use core::fmt::Display;
use num_traits::NumCast;

/// 将两个泛型数值的商转化为百分比字符串，百分比保留指定位小数
pub fn to_percent<T, U>(a: T, b: U, precision: usize) -> String
where
    T: NumCast + Copy,
    U: NumCast + Copy,
{
    let a_val = NumCast::from(a).unwrap_or(0.0);
    let b_val = NumCast::from(b).unwrap_or(0.0);

    let result = if b_val == 0.0 { 0.0 } else { a_val / b_val };
    format!("{:.*}%", precision, result * 100.0)
}

/// 计算两个泛型数值的商，返回格式为"分子/分母(百分比)"的字符串
/// 例如："10/30(33%)"
pub fn to_percent2<T, U>(a: T, b: U) -> String
where
    T: NumCast + Copy + Display,
    U: NumCast + Copy + Display,
{
    let precision = 0;
    let a_str = format!("{}", a);
    let b_str = format!("{}", b);

    let a_val = NumCast::from(a).unwrap_or(0.0);
    let b_val = NumCast::from(b).unwrap_or(0.0);

    let result = if b_val == 0.0 { 0.0 } else { a_val / b_val };
    let percent = format!("{:.*}%", precision, result * 100.0);

    format!("{}/{}({})", a_str, b_str, percent)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_percent() {
        assert_eq!(to_percent(25, 100, 2), "25.00%".to_string());
        assert_eq!(to_percent(1, 3, 2), "33.33%".to_string());
        assert_eq!(to_percent(5.5, 10.0, 2), "55.00%".to_string());
        // 测试不同精度
        assert_eq!(to_percent(1, 4, 0), "25%".to_string());
        assert_eq!(to_percent(1, 3, 4), "33.3333%".to_string());
    }

    #[test]
    fn test_div_format() {
        let a: usize = 10;
        let b: i32 = 30;

        assert_eq!(to_percent2(a, b), "10/30(33%)".to_string());
        assert_eq!(to_percent2(25, 100), "25/100(25%)".to_string());
    }
}
