use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

use crate::variables::HOW_LONG_TO_BEAT_WEBSITE;

pub async fn get_http_response_by_game_name(game_name: String) -> Result<Response<Body>, String> {
    let https_connector = HttpsConnector::new();
    let https_client = Client::builder().build::<_, Body>(https_connector);
    let format_string_with_name = format!("queryString={}&t=games", game_name);

    let how_long_to_beat_website = HOW_LONG_TO_BEAT_WEBSITE.as_str();
    let uri = format!("{}/search_results?page=1", how_long_to_beat_website);
    let request_builder_result = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/x-www-form-urlencoded")
        .header("referer", how_long_to_beat_website)
        .header(
            "user-agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:100.0) Gecko/20100101 Firefox/100.0",
        )
        .body(Body::from(format_string_with_name));

    let request_to_send = match request_builder_result {
        Ok(request_body) => request_body,
        Err(error) => return Err(error.to_string()),
    };

    let http_response = match https_client.request(request_to_send).await {
        Ok(test) => Ok(test),
        Err(error) => Err(error.to_string()),
    };

    http_response
}
