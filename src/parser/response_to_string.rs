use std::error::Error;

use hyper::{body::HttpBody, Body, Response};

pub async fn convert_response_to_html(
    mut response: Response<Body>,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut final_html = String::new();

    while let Some(next) = response.data().await {
        let chunk = next?;

        final_html.push_str(match std::str::from_utf8(&chunk) {
            Ok(val) => val,
            // TODO: Implement more explicit error
            Err(..) => "Error on convert response to HTML.",
        })
    }

    Ok(final_html)
}
