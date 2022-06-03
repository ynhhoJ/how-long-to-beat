use hyper::{Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;

pub async fn get_http_response_by_game_name(game_name: String) -> Result<Response<Body>, String> {
    let https_connector = HttpsConnector::new();
    let https_client = Client::builder().build::<_, Body>(https_connector);
    let format_string_with_name = format!("queryString={}&t=games", game_name);

    let request_builder_result = Request::builder()
        .method(Method::POST)
        .uri("https://howlongtobeat.com/search_results?page=1")
        .header("content-type", "application/x-www-form-urlencoded")
        .header("referer", "https://howlongtobeat.com/")
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
