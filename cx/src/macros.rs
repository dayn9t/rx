
#[macro_export]
macro_rules! try_or_false {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(_) => return false,
    });
}

#[macro_export]
macro_rules! try_or_none {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(_) => return None,
    });
}


#[cfg(test)]
mod tests {


    #[test]
    fn test_decode_option_none() {
        let e: Result<i32, String> = Err(1);


    }
}
