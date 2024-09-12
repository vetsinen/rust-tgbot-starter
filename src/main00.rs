use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use dotenv::dotenv;
use std::env;

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Start the bot.")]
    Start,
    #[command(description = "Stop the bot.")]
    Stop,
}

async fn process_command(bot: Bot, msg: Message, command: Command) -> HandlerResult {
    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Bot started!").await?;
        }
        Command::Stop => {
            bot.send_message(msg.chat.id, "Bot stopped.").await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables from .env file

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot, msg| {
        let command = Command::parse(msg.text().unwrap_or(""), "bot_name").unwrap_or(Command::Help);
        process_command(bot, msg, command)
    })
    .await;
}