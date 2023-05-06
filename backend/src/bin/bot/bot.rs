use commands::HelperCommands;
use commands::QuestionCommands;
use dotenv::dotenv;
use dptree::case;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::RequestError;
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
        bot.send_message(msg.chat.id, "Invalid command").await?;
        return Ok(());
    }

    let current_question = question_manager_global::get_current_question(msg.chat.id.0);
    if (current_question.is_none()) {
        bot.send_message(msg.chat.id, "Sorry, I forgot the question I asked, this usually means it took too long for you to respond, please trigger the question again by running the `/` command").await?;
        return Ok(());
    }

    Ok(())
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
        bot.send_message(msg.chat.id, "Invalid command").await?;
        return Ok(());
    }

    let id = msg.chat.id.0;
    let current_question = question_manager_global::get_current_question(msg.chat.id.0);

    let questions = get_all_questions(command);
    question_manager_global::add_questions(id, questions);

    if (current_question.is_some()) {
        bot.send_message(msg.chat.id, "Okay, but answer my previous question first")
            .await?;
    } else {
        ask_next_question(bot, msg).await?;
    }
    Ok(())
}

async fn on_help(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Help").await?;
    Ok(())
}

async fn on_skip(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Skip").await?;
    Ok(())
}

async fn on_skip_all(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "All questions removed from the queue")
        .await?;

    question_manager_global::remove_all_questions(msg.chat.id.0);
    Ok(())
}

async fn ask_next_question(bot: Bot, msg: Message) -> ResponseResult<()> {
    let id = msg.chat.id.0;
    let question = question_manager_global::get_first_question(id).unwrap();
    bot.send_message(msg.chat.id, question).await?;
    Ok(())
}

fn is_valid_command(command: &str) -> bool {
    command == "/awake"
}

fn get_all_questions(command: &str) -> Vec<String> {
    vec![
        "Did you wake up by yourself?".to_string(),
        "Where are you?".to_string(),
        "What are you doing?".to_string(),
    ]
}
