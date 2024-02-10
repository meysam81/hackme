mod cli;
mod config;
mod errors;
mod httpclient;
mod types;

use crate::errors::Error;
use cli::{Cli, Parser};
use types::{Item, User};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg = config::Settings::new()?;
    println!("{:?}", cfg);

    let cli = Cli::parse();

    let user = httpclient::fetch_user(&cfg.hacker_news_base_url, &cli.user_id).await?;
    println!("{:?}", user);

    match &user.submitted {
        Some(submitted) => {
            for submission in
                httpclient::fetch_submissions(&cfg.hacker_news_base_url, &user).await?
            {
                println!("{:?}", submission);
            }
        }
        None => eprintln!("{} has not submitted any items", user.id),
    }

    Ok(())
}
