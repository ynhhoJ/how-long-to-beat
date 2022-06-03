use crate::utils::number::is_even_number;
use scraper::{ElementRef, Html, Selector};

fn get_link_to_image(element: ElementRef) -> String {
    // get_cover_image
    let cover_image_selector = Selector::parse("img").unwrap();
    let cover_image = element
        .select(&cover_image_selector)
        .next()
        .unwrap()
        .value()
        .attr("src");

    format!("{}{}", "https://howlongtobeat.com", cover_image.unwrap())
}

fn get_how_long_to_beat_time_of_game(element: ElementRef) -> String {
    let game_list_details_selector = Selector::parse(".search_list_details_block").unwrap();
    let game_list_details = element.select(&game_list_details_selector).next().unwrap();

    let text_values_from_detail = game_list_details.text().collect::<Vec<_>>();
    let filtered_values = text_values_from_detail
        .iter()
        .filter(|data| data.contains("\n") == false && data.contains("\t") == false)
        .collect::<Vec<_>>();

    let mut string_for_answer = String::new();

    for (i, el) in filtered_values.iter().enumerate() {
        // NOTE: Every second element is how long to beat game
        if is_even_number(i + 1) {
            string_for_answer.push_str(&format!("{}\n", el).to_string());
        } else {
            string_for_answer.push_str(&format!("{}: ", el).to_string());
        }
    }

    string_for_answer
}

#[derive(Debug)]
pub struct HowLongToBeatResponse {
    image_url: String,
    how_long_to_beat_time: String,
}

pub fn parse_game_data_from_html(html_to_parse: String) -> Vec<HowLongToBeatResponse> {
    let fragment = Html::parse_fragment(&html_to_parse);

    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul = fragment.select(&ul_selector).next().unwrap();
    let mut response: Vec<HowLongToBeatResponse> = Vec::new();

    for element in ul.select(&li_selector) {
        let image_url = get_link_to_image(element);
        let how_long_to_beat_time = get_how_long_to_beat_time_of_game(element);

        response.push(HowLongToBeatResponse {
            image_url,
            how_long_to_beat_time,
        })
    }

    response
}
