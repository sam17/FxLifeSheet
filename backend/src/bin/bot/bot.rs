use commands::HelperCommands;
use commands::QuestionCommands;
use dotenv::dotenv;
use dptree::case;
use teloxide::RequestError;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::{prelude::*};
mod commands;

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

fn schema() -> Handler <'static, DependencyMap, Result<(), RequestError>, DpHandlerDescription> {
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
                .branch(case![HelperCommands::SkillAll].endpoint(on_skill_all)),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dptree::endpoint(message_handler));

    let callback_handler = Update::filter_callback_query()
        .branch(dptree::endpoint(callback_handler));

    let inline_query_handler = Update::filter_inline_query()
        .branch(dptree::endpoint(inline_query_handler));

    let handler = message_handler
        .branch(callback_handler)
        .branch(inline_query_handler);
    handler
}

async fn message_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "Message Handler").await?;
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
    bot.send_message(msg.chat.id, "Question Command").await?;
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

async fn on_skill_all(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "SkillAll").await?;
    Ok(())
}
