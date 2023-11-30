use http::uri::Builder;
use http::uri::Uri;

pub fn stem(url: &str) -> &str {
    let p = url.rfind('/').unwrap() + 1;
    &url[..p]
}

pub fn complete(url: &str, page_url: &str) -> Option<String> {
    if url.starts_with("http") {
        // 全路径
        Some(url.to_string())
    } else if url.starts_with("/") {
        // 站内全路径
        let uri = page_url.parse::<Uri>().ok()?;
        let uri = Builder::new()
            .authority(uri.authority()?.as_str())
            .scheme(uri.scheme_str()?)
            .path_and_query(url)
            .build()
            .ok()?;
        Some(uri.to_string())
    } else {
        // 页面相对路径
        Some(stem(page_url).to_string() + url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stem_works() {
        let url = "/d/a.html";
        assert_eq!("/d/", stem(url));
    }

    #[test]
    fn complete_works() {
        let page = "https://www.biquge.biz/d/";
        let full = "https://www.biquge.biz/d/a.html";

        let url = "/d/a.html";
        assert_eq!(full, &complete(url, page).unwrap());

        let url = "a.html";
        assert_eq!(full, &complete(url, page).unwrap());

        let url = full;
        assert_eq!(full, &complete(url, page).unwrap());

        let page = "https://www.biquge.biz/d/index.html";
        let url = "a.html";
        assert_eq!(full, &complete(url, page).unwrap());
    }
}
