use teloxide::{utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands ask questions:"
)]
pub enum QuestionCommands {
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
}


#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are helper commands:"
)]
pub enum HelperCommands {
    #[command(description = "Shows All Commands.")]
    Help,
     #[command(description = "Skip current question.")]
    Skip,
    #[command(description = "Skill all.")]
    SkipAll,
}