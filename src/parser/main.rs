use super::{
    html_parser::{parse_game_data_from_html, HowLongToBeatResponse},
    http_requster::get_http_response_by_game_name,
    response_to_string::convert_response_to_html,
};

pub async fn main(game_name: String) -> Result<Vec<HowLongToBeatResponse>, String> {
    let http_response_by_game_name = get_http_response_by_game_name(game_name).await;

    let body_response = match http_response_by_game_name {
        Ok(response) => Ok(response),
        Err(error) => return Err(error),
    };

    let body_as_string = match body_response {
        Ok(games_list_raw_html) => convert_response_to_html(games_list_raw_html).await.unwrap(),
        Err(error) => return Err(error),
    };

    let response = parse_game_data_from_html(body_as_string);

    println!("response: {:#?}", response);

    Ok(response)
}
