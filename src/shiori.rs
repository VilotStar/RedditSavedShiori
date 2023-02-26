use crate::reddit::Post;
use reqwest::{Url, header};

pub struct Shiori {
    client: reqwest::Client,
    api: Url
}

impl Shiori {
    pub fn new(api: &str, ses_token: &str) -> Shiori {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Session-Id", ses_token.parse().unwrap());

        let client = reqwest::ClientBuilder::new().default_headers(headers).build().unwrap();
        Shiori {
            client,
            api: api.parse::<Url>().unwrap()
        }
    }

    pub async fn upload(&self, post: &Post) {
        let url = self.api.join("/api/bookmarks").unwrap();

        self.client.post(url).send().await.unwrap();
    }
}
