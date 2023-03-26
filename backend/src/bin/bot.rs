use dotenv::dotenv;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();
    
    let mut questions: Vec<Questions>= Vec::new();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows All Commands.")]
    Help,
    #[command(description = "Track after waking up.")]
    Awake,
    #[command(description = "Track before going to sleep.")]
    Asleep,
    #[command(description = "Track mood.")]
    Mood,
    #[command(description = "Track week.")]
    Week,
    #[command(description = "Track Workouts.")]
    Workout,
    #[command(description = "Skip current question.")]
    Skip,
    #[command(description = "Skill all.")]
    SkillAll
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Awake => bot.send_message(msg.chat.id, "Awake").await?,
        Command::Asleep => bot.send_message(msg.chat.id, "Asleep").await?,
        Command::Mood => bot.send_message(msg.chat.id, "Mood").await?,
        Command::Week => bot.send_message(msg.chat.id, "Week").await?,
        Command::Workout => bot.send_message(msg.chat.id, "Workout").await?,
        Command::Skip => bot.send_message(msg.chat.id, "Skip").await?,
        Command::SkillAll => bot.send_message(msg.chat.id, "SkillAll").await?,
    };
    Ok(())
}

async fn getQuestionsForCommand(command: String) -> Vec<Questions> {
    let mut questions: Vec<Questions> = Vec::new();

    // add 5 questions to the vector
    questions.push(Questions {
        key: "key1".to_string(),
        question: "question1".to_string(),
        question_type: "question_type1".to_string(),
        max_value: Some(1),
        min_value: Some(1),
        buttons: Some("buttons1".to_string()),
        is_positive: true,
        is_reverse: true,
        display_name: "display_name1".to_string(),
    });

    questions.push(Questions {
        key: "key2".to_string(),
        question: "question2".to_string(),
        question_type: "question_type2".to_string(),
        max_value: Some(2),
        min_value: Some(2),
        buttons: Some("buttons2".to_string()),
        is_positive: true,
        is_reverse: true,
        display_name: "display_name2".to_string(),
    });

    questions.push(Questions {
        key: "key3".to_string(),
        question: "question3".to_string(),
        question_type: "question_type3".to_string(),
        max_value: Some(3),
        min_value: Some(3),
        buttons: Some("buttons3".to_string()),
        is_positive: true,
        is_reverse: true,
        display_name: "display_name3".to_string(),
    });

    questions.push(Questions {
        key: "key4".to_string(),
        question: "question4".to_string(),
        question_type: "question_type4".to_string(),
        max_value: Some(4),
        min_value: Some(4),
        buttons: Some("buttons4".to_string()),
        is_positive: true,
        is_reverse: true,
        display_name: "display_name4".to_string(),
    });

    questions.push(Questions {
        key: "key5".to_string(),
        question: "question5".to_string(),
        question_type: "question_type5".to_string(),
        max_value: Some(5),
        min_value: Some(5),
        buttons: Some("buttons5".to_string()),
        is_positive: true,
        is_reverse: true,
        display_name: "display_name5".to_string(),
    });    

    questions
}



struct Questions {
    pub key: String,
    pub question: String,
    pub question_type: String,
    pub max_value: Option<i32>,
    pub min_value: Option<i32>,
    pub buttons: Option<String>,
    pub is_positive: bool,
    pub is_reverse: bool,
    pub display_name: String,
}