use crate::errors::Error;
use crate::types::{Item, User};

pub(crate) async fn fetch_user(base_url: &'_ str, user_id: &'_ str) -> Result<User, Error> {
    let url = &format!("{}/v0/user/{}.json", base_url, user_id);
    let user = reqwest::get(url).await?.json::<User>().await?;

    Ok(user)
}

pub(crate) async fn fetch_submissions(base_url: &'_ str, user: &User) -> Result<Vec<Item>, Error> {
    let mut items = Vec::new();
    for submission in user.submitted.as_ref().unwrap() {
        let url = &format!("{}/v0/item/{}.json", base_url, submission);
        let item = reqwest::get(url).await?.json::<Item>().await?;
        items.push(item);
    }
    Ok(items)
}
