use std::default;

use reqwest::{self, Url};

pub struct Post {
    link: String,
    data_link: String,
    subreddit: String
}

impl Post {
    fn new() {

    }
}

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

    pub async fn get_posts(&self, username: &str) {
        let url = format!("https://old.reddit.com/user/{}/saved?limit=25", username);

        match self.client.get(url).send().await {
            Ok(rep) => {
                Self::scrape_posts(&rep.text().await.unwrap());
            },
            Err(_) => unimplemented!(),
        }
    }

    fn scrape_posts(text: &str) {
        let document = scraper::Html::parse_document(text);
        let posts_sel = scraper::Selector::parse(".thing.saved.link").unwrap();

        let posts: Vec<Post>;
        for element in document.select(&posts_sel) {
            let post = Post {
                link: element.value().attr("data-permalink").unwrap().to_string(),
                data_link: element.value().attr("data-link").unwrap().to_string(),
                subreddit: element.value().attr("data-subreddit").unwrap().to_string(),
            };
            posts.append(post);
        }
    }
}