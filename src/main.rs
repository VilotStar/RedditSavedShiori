use RedditSavePostShiori::{reddit, shiori};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let shiori_session_token = std::env::var("SHIORI_SESSION_TOKEN").expect("'SHIORI_SESSION_TOKEN' must be defined in env!");
    let shiori_api = std::env::var("SHIORI_API").expect("'SHIORI_API' must be defined in env!");
    let reddit_session_token = std::env::var("REDDIT_SESSION_TOKEN").expect("'REDDIT_SESSION_TOKEN' must be defined in env!");
    let username = std::env::var("REDDIT_USERNAME").expect("'REDDIT_USERNAME' must be defined in env!");
    
    let saved = reddit::Saved::new(&reddit_session_token);
    let shiori = shiori::Shiori::new(&shiori_api, &shiori_session_token);
    
    let mut prev_posts = saved.get_posts(&username).await.unwrap();
    loop {
        let cur_posts = saved.get_posts(&username).await.unwrap();
        
        for post in cur_posts.iter() {
            if !prev_posts.contains(&post) {
                shiori.upload(&post).await; // TODO Add return value for error handling
            }
        }

        prev_posts = cur_posts;
        sleep(Duration::from_secs(60 * 20)).await; // Sleep for 20 minutes
    }
}
