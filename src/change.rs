use crate::errors::Error;
use crate::types::{DbData, ItemId};

pub async fn more_comments_added(before: &DbData, after: &DbData) -> Result<Vec<ItemId>, Error> {
    let mut new_comment_ids = vec![];
    for new_item in &after.items {
        for old_item in &before.items {
            if new_item.id == old_item.id && new_item.descendants > old_item.descendants {
                new_comment_ids.push(new_item.id);
            }
        }
    }
    Ok(new_comment_ids)
}
