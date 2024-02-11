mod new_comments;

use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};
use tinytemplate::TinyTemplate;
use tokio::sync::OnceCell;

use crate::errors::Error;
use crate::types::Item;

#[derive(Debug)]
pub(crate) enum Templates {
    NewComments,
}

pub(crate) async fn get_template(template_name: Templates) -> Result<&'static str, Error> {
    match template_name {
        Templates::NewComments => Ok(self::new_comments::get_html().await),
    }
}
