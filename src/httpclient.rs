use crate::errors::Error;
use crate::types::{Item, User};

pub(crate) async fn fetch_user<'a>(base_url: &'a str, user_id: &'a str) -> Result<User, Error> {
    let url = &format!("{}/v0/user/{}.json", base_url, user_id);
    let user = reqwest::get(url).await?.json::<User>().await?;

    Ok(user)
}

pub(crate) async fn fetch_submissions<'a>(
    base_url: &'a str,
    user: &User,
) -> Result<Vec<Item>, Error> {
    let mut items = Vec::new();
    if let Some(submitted) = &user.submitted {
        for submission in submitted {
            let url = &format!("{}/v0/item/{}.json", base_url, submission);
            let item = reqwest::get(url).await?.json::<Item>().await?;
            items.push(item);
        }
    }
    Ok(items)
}
