use crate::errors::Error;
use crate::types::DbData;
use serde_json::to_string;

use tokio::fs::File;
use tokio::io::{self, AsyncWriteExt};

pub(crate) async fn write_db(db_data: &'_ DbData, db_url: &'_ str) -> Result<(), Error> {
    let mut file = File::create(db_url).await?;
    file.write_all(to_string(db_data)?.as_bytes()).await?;
    Ok(())
}
