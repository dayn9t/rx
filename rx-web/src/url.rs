use http::uri::Builder;
use http::uri::Uri;

pub fn complete(url: &str, page_url: &str) -> String {
    if url.starts_with("http") {
        url.to_string()
    } else if url.starts_with("/") {
        let uri = page_url.parse::<Uri>().unwrap();
        let uri = Builder::new()
            .authority(uri.authority_part().unwrap().as_str())
            .scheme(uri.scheme_str().unwrap())
            .path_and_query(url)
            .build()
            .unwrap();
        uri.to_string()
    } else {
        page_url.to_string() + url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_works() {
        let page = "https://www.biquge.biz/d/";
        let full = "https://www.biquge.biz/d/a.html";

        let url = "/d/a.html";
        assert_eq!(full, &complete(url, page));

        let url = "a.html";
        assert_eq!(full, &complete(url, page));

        let url = full;
        assert_eq!(full, &complete(url, page));
    }
}
