use reqwest::{self, Response, Url};

pub struct Saved {
    client: reqwest::Client
}

impl Saved {
    pub fn new(ses_token: &str) -> Saved {
        let cookies = reqwest::cookie::Jar::default();
        
        {
            let cookie = format!("reddit_session={}; Domain=reddit.com", ses_token);
            let url = "https://reddit.com".parse::<Url>().unwrap();
            cookies.add_cookie_str(&cookie, &url);
        }
    
        let client = reqwest::ClientBuilder::new().cookie_store(true).cookie_provider(cookies.into()).build().unwrap();
        Saved {
            client
        }
    }

    pub async fn get_posts(&self, username: &str) -> Option<Response> {
        let url = format!("https://old.reddit.com/user/{}/saved?limit=25", username);

        let req = self.client.get(url).send().await;
        req
    }
}