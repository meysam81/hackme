//! Hack Me
//!

#![feature(async_closure)]

mod change;
mod cli;
mod config;
mod errors;
mod httpclient;
mod persistence;
mod template;
mod types;

use crate::cli::{Cli, Parser};
use crate::errors::Error;
use crate::persistence::{read_db, write_db};
use crate::template::{get_template, Templates};
use crate::types::DbData;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cfg = config::Settings::new()?;

    let cli = Cli::parse();

    let old_db = read_db(&cfg.db_url).await?;

    let mut new_db: DbData = Default::default();

    let user = httpclient::fetch_user(&cfg.hacker_news_base_url, &cli.user_id).await?;

    if let Some(submitted) = &user.submitted {
        new_db.items = httpclient::fetch_submissions(&cfg.hacker_news_base_url, submitted).await?;
    }
    new_db.user = user;

    let _new_comment_ids = change::more_comments_added(&old_db, &new_db).await?;
    // let new_comments: Vec<Item> = new_db
    //     .items
    //     .iter()
    //     .filter(|i| new_comment_ids.contains(&i.id))
    //     .cloned()
    //     .collect();

    // println!("New comments: {:?}", new_comments);

    write_db(&new_db, &cfg.db_url).await?;

    let _tpl = get_template(Templates::NewComments).await?;
    // println!("{}", tpl);

    Ok(())
}
