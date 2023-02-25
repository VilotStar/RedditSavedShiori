use RedditSavePostShiori::reddit::{self as reddit, Saved};
use scraper;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let session_token = std::env::var("REDDIT_SESSION_TOKEN").expect("'REDDIT_SESSION_TOKEN' must be defined in env!");
    let username = std::env::var("REDDIT_USERNAME").expect("'REDDIT_USERNAME' must be defined in env!");
    
    let saved = reddit::Saved::new(&session_token);
    
    saved.get_posts(&username).await;
}
