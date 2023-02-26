use crate::reddit::Post;
use reqwest::{Url, header};

struct Shiori {
    client: reqwest::Client,
    api: String
}

impl Shiori {
    pub fn new(api: String, ses_token: &str) -> Shiori {
        let mut headers = header::HeaderMap::new();
        headers.insert("X-Session-Id", header::HeaderValue::from(ses_token)); // Fix later or juse use .header in later fucntions

        let client = reqwest::ClientBuilder::new().default_headers(headers).build().unwrap();
        Shiori {
            client,
            api
        }
    }

    pub async fn upload(&self, post: Post) {
        let url = format!("{}api/bookmarks", self.api);

        self.client.post(url).send().await.unwrap();
    }
}
