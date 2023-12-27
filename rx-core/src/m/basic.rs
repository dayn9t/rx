
use geo_types::CoordNum;

/// 不同类型的除法, 用f64计算中间结果
pub fn div_f64<T1: CoordNum, T2: CoordNum, D: CoordNum>(a: T1, b: T2) -> Option<D> {
    D::from(a.to_f64()? / b.to_f64()?)
}

/// 不同类型的乘法, 用f64计算中间结果, 舍入成整数
pub fn mul_round_f64<T1: CoordNum, T2: CoordNum, D: CoordNum>(a: T1, b: T2) -> Option<D> {
    let s = a.to_f64()? * b.to_f64()?;
    D::from(s.round())
}

/// 偏序最小值
pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

/// 偏序最大值
pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    if b >= a {
        b
    } else {
        a
    }
}
