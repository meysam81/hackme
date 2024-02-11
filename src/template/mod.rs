mod new_comments;

use crate::errors::Error;

#[derive(Debug)]
pub(crate) enum Templates {
    NewComments,
}

pub(crate) async fn get_template(template_name: Templates) -> Result<&'static str, Error> {
    match template_name {
        Templates::NewComments => Ok(new_comments::get_html().await),
    }
}
