mod cli;
mod config;
mod errors;
mod httpclient;
mod persistence;
mod types;

use crate::errors::Error;
use cli::{Cli, Parser};
use persistence::{write_db, read_db};
use types::{DbData, Item, User};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg = config::Settings::new()?;
    println!("{:?}", cfg);

    let cli = Cli::parse();

    let mut db_data: DbData = Default::default();

    let user = httpclient::fetch_user(&cfg.hacker_news_base_url, &cli.user_id).await?;
    println!("{:?}", user);

    db_data.user = user.clone();

    match &user.submitted {
        Some(submitted) => {
            let user_submissions =
                httpclient::fetch_submissions(&cfg.hacker_news_base_url, &user).await?;
            for submission in &user_submissions {
                println!("{:?}", submission);
            }
            db_data.items = user_submissions;
        }
        None => eprintln!("{} has not submitted any items", user.id),
    }

    write_db(&db_data, &cfg.db_url).await?;

    let db = read_db(&cfg.db_url).await?;
    println!("{:?}", db);

    Ok(())
}
