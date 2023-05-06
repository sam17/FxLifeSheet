use std::vec;
use models::models::questions::viz_questions::Question;
use commands::HelperCommands;
use commands::QuestionCommands;
use dotenv::dotenv;
use dptree::case;
use models::models::questions::viz_questions::QuestionKey;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::RequestError;
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
    let message_text = msg.text().unwrap();
    if message_text.starts_with("/") {
        bot.send_message(msg.chat.id, "Invalid command, Try the following").await?;
        on_help(bot, msg).await?;
        return Ok(());
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
        "text" => {
            add_answer_to_db(msg.text().unwrap());
            ask_next_question(bot, msg).await?;
            Ok(())
        }
        "number" => {
            let answer = msg.text().unwrap();
            if !answer.parse::<i32>().is_ok() {
                bot.send_message(msg.chat.id, "Invalid number, please try again").await?;
                return Ok(());
            }
            add_answer_to_db(msg.text().unwrap());
            ask_next_question(bot, msg).await?;
            Ok(())
         }
        "range" => {
        add_answer_to_db(msg.text().unwrap());
            ask_next_question(bot, msg).await?;
            Ok(())
     
        }
        _ => {
            bot.send_message(msg.chat.id, "Sorry, I don't know how to handle this answer type").await?;
            Ok(())
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
        send_range_options(&bot, msg.chat.id, question.question.as_str()).await?;
        return Ok(());
    }

    if question.answer_type == "boolean" {
        send_boolean_options(&bot, msg.chat.id, question.question.as_str()).await?;
        return Ok(());
    }

    if question.answer_type == "location" {
  
    }

    bot.send_message(msg.chat.id, question.question).await?;
    Ok(())
}

async fn send_range_options(bot: &Bot, chat_id: ChatId, question_text: &str) -> ResponseResult<()> {
    let options: Vec<String> = (1..=5).map(|i| i.to_string()).collect();

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

fn get_all_questions(command: &str) -> Vec<Question> {
    vec![
        Question {
            id: 1,
            key: QuestionKey("name".to_string()),
            question: "What is your name?".to_string(),
            answer_type: "text".to_string(),
            parent_question: None,
            parent_question_option: None,
            category: None,
            max: None,
            min: None,
            show: false,
            display_name: "Name".to_string(),
            is_positive: true
    },
    Question {
        id: 2,
        key: QuestionKey("age".to_string()),
        question: "What is your age?".to_string(),
        answer_type: "range".to_string(),
        parent_question: None,
        parent_question_option: None,
        category: None,
        max: None,
        min: None,
        show: false,
        display_name: "Age".to_string(),
        is_positive: true
    }
    ]

}
