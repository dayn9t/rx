use anyhow::anyhow;
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_LENGTH;
use rx_core::prelude::{Deserialize, Serialize};
use rx_core::text::AnyResult;
use std::fs;
use std::path::Path;
use url::Url;

pub const PROTO_HTTP: &str = "http://";
pub const PROTO_HTTPS: &str = "https://";
pub const PROTO_FILE: &str = "file://";

/// URL 分解成 base 和 path 两部分
pub fn split_url2(url: impl AsRef<str>) -> AnyResult<(String, String)> {
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

/// 文件MIME信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMimeInfo {
    pub size: usize,
    pub content_type: String,
}

/// 获取文件信息
pub fn get_file_mime_info(path: &Path) -> AnyResult<FileMimeInfo> {
    let metadata = fs::metadata(path)?;

    let size = metadata.len() as usize;
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    Ok(FileMimeInfo { size, content_type })
}

/// 获取 HTTP 文件信息
pub fn get_http_mime_info(url: &str) -> AnyResult<FileMimeInfo> {
    let client = Client::new();
    let response = client.head(url).send()?;

    if response.status() != StatusCode::OK {
        return Err(anyhow!("Failed to fetch URL: {:?}", response.status()));
    }

    let size = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    Ok(FileMimeInfo { size, content_type })
}

/// 获取 URL 文件信息， 目前只支持 HTTP/FILE 协议
pub fn get_url_mime_info(url: impl AsRef<str>) -> AnyResult<FileMimeInfo> {
    let url_str = url.as_ref();
    if url_str.starts_with(PROTO_HTTP) || url_str.starts_with(PROTO_HTTPS) {
        get_http_mime_info(url_str)
    } else if let Some(stripped) = url_str.strip_prefix(PROTO_FILE) {
        let path = Path::new(stripped);
        get_file_mime_info(path)
    } else {
        Err(anyhow!("Unsupported URL: {}", url_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_url2() {
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

    use std::fs::File;
    use std::io::Write;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            // Create a temporary file for testing file URL
            let mut file = File::create("/tmp/test_file.txt").unwrap();
            writeln!(file, "Hello, world!").unwrap();
        });
    }

    #[test]
    fn test_get_url_mime_info_http() {
        setup();
        //let url = "http://example.com";
        let url = "http://localhost/static/projects/s4/video/2024-11-12/C1_2024_11_12T15_19_53.mkv";

        let result = get_url_mime_info(url);
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(info.size > 0);
        assert!(!info.content_type.is_empty());
    }

    #[test]
    fn test_get_url_mime_info_file() {
        setup();
        let file_url = "file:///tmp/test_file.txt";

        let result = get_url_mime_info(file_url);
        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.content_type, "text/plain");
    }

    #[test]
    fn test_get_url_mime_info_unsupported() {
        setup();
        let unsupported_url = "ftp://example.com";
        let result = get_url_mime_info(unsupported_url);
        assert!(result.is_err());
    }
}
