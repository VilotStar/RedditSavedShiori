use RedditSavePostShiori::reddit::{self as reddit, Saved};
use scraper;
use tokio;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let session_token = std::env::var("SESSION_TOKEN").expect("'SESSION_TOKEN' must be defined in env!");
    let username = std::env::var("USERNAME").expect("'USERNAME' must be defined in env!");
    
    let saved = reddit::Saved::new(&session_token);
    
    match saved.get_posts(&username) {

    }
}
