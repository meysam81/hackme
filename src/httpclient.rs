use crate::errors::Error;
use crate::types::{Item, ItemId, User};

use indicatif::ProgressBar;
use std::future::Future;
use tokio::task::JoinSet;

pub(crate) async fn fetch_user<'a>(base_url: &'a str, user_id: &'a str) -> Result<User, Error> {
    let url = &format!("{}/v0/user/{}.json", base_url, user_id);
    let user = reqwest::get(url).await?.json::<User>().await?;

    Ok(user)
}

pub(crate) async fn process_batch(
    batch: Vec<impl Future<Output = Result<Item, Error>> + 'static + std::marker::Send>,
) -> Result<Vec<Item>, Error> {
    let mut items = Vec::new();
    let mut set: JoinSet<Result<Item, Error>> = JoinSet::new();

    for task in batch {
        set.spawn(task);
    }

    while let Some(result) = set.join_next().await {
        items.push(result??);
    }

    Ok(items)
}

pub(crate) async fn fetch_submissions<'a>(
    base_url: &'a str,
    submissions: &Vec<ItemId>,
) -> Result<Vec<Item>, Error> {
    let mut items = Vec::new();

    let batch_size = 2_usize.pow(8);
    let mut batch = Vec::new();
    let sub_size: usize = submissions.len();
    let bar = ProgressBar::new(sub_size.try_into().unwrap());

    println!("Fetching user submissions...");
    for submission in submissions {
        bar.inc(1);
        let url = format!("{}/v0/item/{}.json", base_url, submission);
        let task = async move {
            let item = reqwest::get(url).await?.json::<Item>().await?;
            Ok(item)
        };
        batch.push(task);

        if batch.len() >= batch_size {
            items.extend(process_batch(batch).await?);
            batch = Vec::new();
        }
    }
    if !batch.is_empty() {
        items.extend(process_batch(batch).await?);
    }
    bar.finish_and_clear();
    println!("Successfully fetched {} submissions", sub_size);

    Ok(items)
}
