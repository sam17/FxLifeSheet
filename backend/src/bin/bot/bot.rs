use std::println;
use std::vec;
use models::models::questions::question_options::QuestionOption;
use models::models::questions::viz_questions::Question;
use commands::HelperCommands;
use commands::QuestionCommands;
use dotenv::dotenv;
use dptree::case;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::types::ButtonRequest;
use teloxide::types::Location;
use teloxide::types::{KeyboardButton, KeyboardMarkup};
use teloxide::utils::command::BotCommands;
mod commands;
mod question_manager_global;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();
    Dispatcher::builder(bot, schema())
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> Handler<'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
    let command_handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<QuestionCommands>()
                .endpoint(on_question_command),
        )
        .branch(
            dptree::entry()
                .filter_command::<HelperCommands>()
                .branch(case![HelperCommands::Help].endpoint(on_help))
                .branch(case![HelperCommands::Skip].endpoint(on_skip))
                .branch(case![HelperCommands::SkipAll].endpoint(on_skip_all)),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(message_handler));

    let callback_handler =
        Update::filter_callback_query().branch(dptree::endpoint(callback_handler));

    let inline_query_handler =
        Update::filter_inline_query().branch(dptree::endpoint(inline_query_handler));

    let handler = message_handler
        .branch(callback_handler)
        .branch(inline_query_handler);
    handler
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(message_text) = msg.text() {
    if message_text.starts_with("/") {
        bot.send_message(msg.chat.id, "Invalid command, Try the following").await?;
        on_help(bot, msg).await?;
        return Ok(());
    }
    }

    let current_question = question_manager_global::get_current_question(msg.chat.id.0);
    
    match current_question {
        Some(question) => {
            handle_answer(bot, msg, question).await?;
            return Ok(());
        }
        None => {
            bot.send_message(msg.chat.id, "Sorry, I forgot the question I asked, this usually means it took too long for you to respond, please trigger the question again by running the `/` command").await?;
            return Ok(());
        }
    }
}

async fn handle_answer(bot: Bot, msg: Message, question: Question) -> ResponseResult<()> {
    match question.answer_type.as_str() {
        "text" | "range" | "boolean"  => {
            on_question_answered(bot, msg, question).await?;
            Ok(())
        }
        "number" => {
            let answer = msg.text().unwrap();
            if !answer.parse::<i32>().is_ok() {
                bot.send_message(msg.chat.id, "Invalid number, please try again").await?;
                return Ok(());
            }
            on_question_answered(bot, msg, question).await?;
            Ok(())
         }
        "location" => {
            println!("location added");
            if let Some(location) = msg.location() {
                add_location_to_db(location);
            } else {
                bot.send_message(msg.chat.id, "Invalid location, please try again").await?;
                return Ok(());
            }
            ask_next_question(bot, msg).await?;
            Ok(())
        }

        _ => {
            bot.send_message(msg.chat.id, "Sorry, I don't know how to handle this answer type").await?;
            Ok(())
        }
    }
}

async fn on_question_answered(bot: Bot, msg: Message, question: Question) -> ResponseResult<()> {
    add_answer_to_db(msg.text().unwrap());
    add_follow_up_question(question, &msg);
    ask_next_question(bot, msg).await?;
    Ok(()) 
}

fn add_follow_up_question(question: Question, msg: &Message) {
    let message_text = msg.text().unwrap();
    let user_id = msg.chat.id.0;

    if let Some(options) = question.question_options {
        if let Some(option) = options.into_iter().find(|option| option.name == message_text) {
            
            let new_questions = get_question_for_option(option.question_key, option.id);
        
            question_manager_global::add_questions_to_front(user_id, new_questions)
        } 
    }
}

async fn callback_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Callback Handler").await?;
    Ok(())
}

async fn inline_query_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Inline Query Handler")
        .await?;
    Ok(())
}

async fn on_question_command(bot: Bot, msg: Message) -> ResponseResult<()> {
    let command = msg.text().unwrap();
    if !is_valid_command(command) {
        bot.send_message(msg.chat.id, "Invalid command, try the following").await?;
        on_help(bot, msg).await?;
        return Ok(());
    }

    let id = msg.chat.id.0;
    let current_question = question_manager_global::get_current_question(msg.chat.id.0);

    let questions = get_all_questions(command);
    question_manager_global::add_questions(id, questions);

    if current_question.is_some() {
        bot.send_message(msg.chat.id, "Okay, but answer my previous question first")
            .await?;
    } else {
        ask_next_question(bot, msg).await?;
    }
    Ok(())
}

async fn on_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    let help_text = HelperCommands::descriptions().to_string() + "\n\n" + QuestionCommands::descriptions().to_string().as_str();    
    bot.send_message(msg.chat.id, help_text).await?;
    Ok(())
}

async fn on_skip(bot: Bot, msg: Message) -> ResponseResult<()> {
    let current_question = question_manager_global::get_current_question(msg.chat.id.0);
    if current_question.is_none() {
        bot.send_message(msg.chat.id, "No question to skip").await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Skipping the question").await?;
    ask_next_question(bot, msg).await?;

    Ok(())
}

async fn on_skip_all(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "All questions removed from the queue")
        .await?;

    question_manager_global::remove_all_questions(msg.chat.id.0);
    Ok(())
}

async fn ask_next_question(bot: Bot, msg: Message) -> ResponseResult<()> {
    if question_manager_global::is_question_queue_empty(msg.chat.id.0) {
        question_manager_global::set_current_question_nulled(msg.chat.id.0);
        bot.send_message(msg.chat.id, "All done for now").await?;
        return Ok(());
    }

    let id = msg.chat.id.0;
    let question = question_manager_global::get_first_question(id).unwrap();

    if question.answer_type == "range" {
        send_range_options(&bot, msg.chat.id, question.question.as_str(), question.question_options).await?;
        return Ok(());
    }

    if question.answer_type == "boolean" {
        send_boolean_options(&bot, msg.chat.id, question.question.as_str()).await?;
        return Ok(());
    }

    if question.answer_type == "location" {
        send_location_options(&bot, msg.chat.id, question.question.as_str()).await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, question.question).await?;
    Ok(())
}

async fn send_range_options(bot: &Bot, chat_id: ChatId, question_text: &str, question_options: Option<Vec<QuestionOption>>) -> ResponseResult<()> {
    let options: Vec<String> = match question_options {
        Some(options) => options.iter().map(|option| option.name.clone()).collect(),
        None => vec![], // return an empty vector if question_options is None
    };

    let keyboard = make_keyboard(options);

    bot.send_message(chat_id, question_text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn send_boolean_options(bot: &Bot, chat_id: ChatId, question_text: &str) -> ResponseResult<()> {
    let options: Vec<String> = vec!["Yes".to_string(), "No".to_string()];

    let keyboard = make_keyboard(options);

    bot.send_message(chat_id, question_text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

async fn send_location_options(bot: &Bot, chat_id: ChatId, question_text: &str) -> ResponseResult<()> {
    let keyboard = make_location_keyboard();

    bot.send_message(chat_id, question_text)
        .reply_markup(keyboard)
        .await?;

    Ok(())
}

fn make_location_keyboard() -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    let button = KeyboardButton::new("Share Location".to_owned()).request(ButtonRequest::Location);
    keyboard.push(vec![button]);

    KeyboardMarkup::new(keyboard).one_time_keyboard(true)
}

fn make_keyboard(options: Vec<String>) -> KeyboardMarkup {
    let mut keyboard: Vec<Vec<KeyboardButton>> = vec![];

    for option in options.chunks(3) {
        let row = option
            .iter()
            .map(|version| KeyboardButton::new(version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    KeyboardMarkup::new(keyboard).one_time_keyboard(true)
}

fn is_valid_command(command: &str) -> bool {
    command == "/awake"
}

fn add_answer_to_db(answer: &str) {
    println!("Answer: {}", answer)
}

fn add_location_to_db(location: &Location) {
    println!("Location: {:?}", location)
}

fn get_question_for_option(parent_question_key: String, parent_question_option:i32) -> Vec<Question> {

    if parent_question_option != 2 {
        return vec![];
    }    


    vec![
        Question {
            id: 3,
            key: "mood".to_string(),
            question: "What is your mood?".to_string(),
            answer_type: "range".to_string(),
            parent_question: Some("name".to_string()),
            parent_question_option: Some("Yes".to_string()),
            category: None,
            max: Some(100),
            min: Some(0),
            show: false,
            display_name: "Age".to_string(),
            is_positive: true,
            cadence: "daily".to_string(),
            command: None,
            graph_type: "Line".to_string(),
            question_options: Some(vec![
                QuestionOption {
                    id: 1,
                    name: "Yes".to_string(),
                    question_key: "3".to_string(),
                },
                QuestionOption {
                    id: 2,
                    name: "No".to_string(),
                    question_key: "3".to_string(),
                },
            ]),
        },
    ]
}

fn get_all_questions(command: &str) -> Vec<Question> {
    vec![
        Question {
            id: 1,
            key: "name".to_string(),
            question: "What is your name?".to_string(),
            answer_type: "text".to_string(),
            parent_question: None,
            parent_question_option: None,
            category: None,
            max: None,
            min: None,
            show: false,
            display_name: "Name".to_string(),
            is_positive: true,
            cadence: "daily".to_string(),
            command: None,
            graph_type: "Line".to_string(),
            question_options: None
    },
    Question {
        id: 2,
        key: "age".to_string(),
        question: "What is your age?".to_string(),
        answer_type: "range".to_string(),
        parent_question: None,
        parent_question_option: None,
        category: None,
        max: None,
        min: None,
        show: false,
        display_name: "Age".to_string(),
        is_positive: true,
            cadence: "daily".to_string(),
            command: None,
            graph_type: "Line".to_string(),
            question_options: Some(vec![
                QuestionOption { 
                    id: 1,
                    name: "0-10".to_string(),
                    question_key: "age".to_string(),
                },
                QuestionOption { 
                    id: 2,
                    name: "11-20".to_string(),
                    question_key: "age".to_string(),
                },
                QuestionOption { 
                    id: 3,
                    name: "21-30".to_string(),
                    question_key: "age".to_string(),
                },
            ])
     }
    ]

}
