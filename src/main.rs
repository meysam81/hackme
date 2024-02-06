mod cli;
mod types;

use cli::{Cli, Parser};
use types::{Item, User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let url = &format!(
        "https://hacker-news.firebaseio.com/v0/user/{}.json",
        cli.user_id
    );
    let user = reqwest::get(url).await?.json::<User>().await?;
    println!("{:?}", user);

    for submission in &user.submitted {
        let url = &format!(
            "https://hacker-news.firebaseio.com/v0/item/{}.json",
            submission
        );
        let item = reqwest::get(url).await?.json::<Item>().await?;

        println!("{:?}", item);
    }

    Ok(())
}
