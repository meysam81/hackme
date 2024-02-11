use crate::errors::Error;
use crate::types::{Item, ItemId, User};
use std::future::Future;
use tokio::task::JoinSet;

pub(crate) async fn fetch_user<'a>(base_url: &'a str, user_id: &'a str) -> Result<User, Error> {
    let url = &format!("{}/v0/user/{}.json", base_url, user_id);
    let user = reqwest::get(url).await?.json::<User>().await?;

    Ok(user)
}

pub(crate) async fn process_batch<'a>(
    base_url: &'a str,
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

    let mut batch = Vec::new();

    for submission in submissions {
        let url = String::from(format!("{}/v0/item/{}.json", base_url, submission));
        let task = async move {
            let item = reqwest::get(url).await?.json::<Item>().await?;
            Ok(item)
        };
        batch.push(task);

        if batch.len() == 50 {
            items.extend(process_batch(base_url, batch).await?);
            batch = Vec::new();
        }
    }
    if !batch.is_empty() {
        items.extend(process_batch(base_url, batch).await?);
    }
    Ok(items)
}
