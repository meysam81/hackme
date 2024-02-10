mod cli;
mod config;
mod types;

use cli::{Cli, Parser};
use types::{Item, User};

async fn fetch_submissions(user: &User) -> Result<Vec<Item>, Box<dyn std::error::Error>> {
    let mut items = Vec::new();
    for submission in user.submitted.as_ref().unwrap() {
        let url = &format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            submission
        );
        let item = reqwest::get(url).await?.json::<Item>().await?;
        items.push(item);
    }
    Ok(items)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Settings::new()?;
    println!("{:?}", cfg);

    let cli = Cli::parse();

    let url = &format!("{}/v0/user/{}.json", cfg.hacker_news_base_url, cli.user_id);
    let user = reqwest::get(url).await?.json::<User>().await?;
    println!("{:?}", user);

    match &user.submitted {
        Some(submitted) => {
            for submission in fetch_submissions(&user).await? {
                println!("{:?}", submission);
            }
        }
        None => eprintln!("{} has not submitted any items", user.id),
    }

    Ok(())
}
