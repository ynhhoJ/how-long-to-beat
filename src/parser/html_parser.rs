use crate::utils::number::is_even_number;
use scraper::{ElementRef, Html, Selector};

fn get_link_to_image(element: ElementRef) -> String {
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
    let game_details_selector = Selector::parse(".search_list_details_block").unwrap();
    let game_details_element = element.select(&game_details_selector).next().unwrap();

    let vector_with_game_details = game_details_element.text().collect::<Vec<_>>();
    let filtered_game_details = vector_with_game_details
        .iter()
        .filter(|data| !data.contains('\n') && !data.contains('\t'))
        .collect::<Vec<_>>();

    let mut string_with_hours_to_beat_the_game = String::new();

    for (i, el) in filtered_game_details.iter().enumerate() {
        // NOTE:
        //      ODD number is title of category, like "Main Story", "Main + Extra" etc.
        //      EVEN number is needed time to complete the game. Idk at this moment how this part of code
        //      can be written better.
        let is_title = !is_even_number(i + 1);

        if is_title {
            string_with_hours_to_beat_the_game.push_str(&format!("<b>{}</b>: ", el).to_string());
        } else {
            string_with_hours_to_beat_the_game.push_str(&format!("{}\n", el).to_string());
        }
    }

    string_with_hours_to_beat_the_game
}

fn get_title_of_game(element: ElementRef) -> String {
    let title_selector = Selector::parse(".search_list_details h3 a").unwrap();
    let game_title = element.select(&title_selector).next().unwrap();

    match game_title.text().next() {
        Some(data) => data.to_string(),
        None => "undefined".to_string(),
    }
}

#[derive(Debug)]
pub struct HowLongToBeatResponse {
    pub how_long_to_beat_time: String,
    pub image_url: String,
    pub title: String,
}

pub fn parse_game_data_from_html(html_to_parse: String) -> Vec<HowLongToBeatResponse> {
    let fragment = Html::parse_fragment(&html_to_parse);

    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul_option = fragment.select(&ul_selector).next();
    let mut how_long_to_beat_response: Vec<HowLongToBeatResponse> = Vec::new();

    let ul = match ul_option {
        Some(data) => data,
        // NOTE: If `ul_option` is `None` as response will be returned empty vector.
        None => return how_long_to_beat_response,
    };

    for element in ul.select(&li_selector) {
        let image_url = get_link_to_image(element);
        let how_long_to_beat_time = get_how_long_to_beat_time_of_game(element);
        let title = get_title_of_game(element);

        how_long_to_beat_response.push(HowLongToBeatResponse {
            title,
            image_url,
            how_long_to_beat_time,
        })
    }

    how_long_to_beat_response
}
