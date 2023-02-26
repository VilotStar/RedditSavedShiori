use reqwest::{self, Url};

pub struct Post {
    link: String,
    data_link: String,
    subreddit: String
}

impl PartialEq for Post {
    fn eq(&self, other: &Self) -> bool {
        self.link == other.link
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

    pub async fn get_posts(&self, username: &str) -> Option<Vec<Post>> {
        let url = format!("https://old.reddit.com/user/{}/saved?limit=25", username);

        match self.client.get(url).send().await {
            Ok(rep) => {
                Some(Self::scrape_posts(&rep.text().await.unwrap()))
            },
            Err(_) => {
                None
            }
        }
    }

    fn scrape_posts(text: &str) -> Vec<Post> {
        let document = scraper::Html::parse_document(text);
        let posts_sel = scraper::Selector::parse(".thing.saved.link").unwrap();

        let posts: Vec<Post> = document.select(&posts_sel).into_iter().map(|ele| { 
            Post {
                link: ele.value().attr("data-permalink").unwrap().to_string(),
                data_link: ele.value().attr("data-link").unwrap().to_string(),
                subreddit: ele.value().attr("data-subreddit").unwrap().to_string()
            }
        }).rev().collect();
        posts
    }
}