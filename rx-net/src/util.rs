use std::error::Error;

use url::Url;

/// URL 分解成 base 和 path 两部分
pub fn split_url2(url: impl AsRef<str>) -> Result<(String, String), Box<dyn Error>> {
    let parsed_url = Url::parse(url.as_ref())?;
    let base = match (parsed_url.username(), parsed_url.password()) {
        ("", None) => format!(
            "{}://{}",
            parsed_url.scheme(),
            parsed_url.host_str().unwrap()
        ),
        (user, Some(pass)) => format!(
            "{}://{}:{}@{}",
            parsed_url.scheme(),
            user,
            pass,
            parsed_url.host_str().unwrap()
        ),
        (user, None) => format!(
            "{}://{}@{}",
            parsed_url.scheme(),
            user,
            parsed_url.host_str().unwrap()
        ),
    };
    let base = if let Some(port) = parsed_url.port() {
        format!("{}:{}", base, port)
    } else {
        base
    };
    let path = parsed_url.path().to_string();
    Ok((base, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = [
            [
                "http://www.baidu.com/abc/def",
                "http://www.baidu.com",
                "/abc/def",
            ],
            [
                "rtsp://admin:howell1409@10.1.0.21:553/Streaming/Channels/101",
                "rtsp://admin:howell1409@10.1.0.21:553",
                "/Streaming/Channels/101",
            ],
        ];

        for item in data.iter() {
            let url = item[0];
            let (base, path) = split_url2(url).unwrap();
            assert_eq!(base, item[1]);
            assert_eq!(path, item[2]);
        }
    }
}
