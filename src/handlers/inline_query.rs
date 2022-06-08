use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
        ParseMode,
    },
    RequestError,
};

use crate::parser::{html_parser::HowLongToBeatResponse, main::main as parser_game_info_main};
use crate::types::bot::BotWithAutoSendAndThrottle;

fn get_teloxide_result_article(
    parser_result: Vec<HowLongToBeatResponse>,
) -> Vec<InlineQueryResult> {
    // NOTE: Is used to create unique Id for teloxide article result
    let mut article_map_index = 0;

    parser_result
        .iter()
        .map(|data| {
            let inline_query_result_article = InlineQueryResultArticle::new(
                article_map_index.to_string(),
                &data.title,
                InputMessageContent::Text(
                    InputMessageContentText::new(format!(
                        "{}\n\n<b>{}</b>\n\n{}",
                        &data.image_url, &data.title, &data.how_long_to_beat_time
                    ))
                    .parse_mode(ParseMode::Html),
                ),
            )
            .thumb_url(data.image_url.parse().unwrap());

            article_map_index += 1;

            InlineQueryResult::Article(inline_query_result_article)
        })
        .collect::<Vec<InlineQueryResult>>()
}

pub async fn inline_query() -> Handler<
    'static,
    DependencyMap,
    Result<(), RequestError>,
    teloxide::dispatching::DpHandlerDescription,
> {
    let handler = Update::filter_inline_query().branch(dptree::endpoint(
        |query: InlineQuery, bot: BotWithAutoSendAndThrottle| async move {
            let game_name_to_search = &query.query;
            let parser_response = parser_game_info_main(game_name_to_search.to_string()).await;

            let parser_result = match parser_response {
                Ok(result) => result,
                Err(..) => Vec::new(),
            };

            let teloxide_result_article = get_teloxide_result_article(parser_result);
            let response = bot
                .answer_inline_query(&query.id, teloxide_result_article)
                .send()
                .await;

            if let Err(err) = response {
                println!("Error in handler: {:?}", err)
            }

            respond(())
        },
    ));

    handler
}
