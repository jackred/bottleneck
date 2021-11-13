use teloxide::types;
use teloxide::{prelude::*, utils::command::BotCommand};

extern crate chrono;
use chrono::{NaiveDateTime};

use serde_json;
use std::fs::File;
use std::io::Read;
use std::error::Error;

#[tokio::main]
async fn main() {
    run().await;
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "send a poll with all days of the week")]
    Poll,
    #[command(description = "check if the bot is alive")]
    Ping	
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {cx.answer(Command::descriptions()).await?;}
	Command::Poll => {
	    let options = vec![
    		String::from("Lundi"),
    		String::from("Mardi"),
    		String::from("Mercredi"),
    		String::from("Jeudi"),
    		String::from("Vendredi"),
    		String::from("Samedi"),
    		String::from("Dimanche"),
    	    ];
	    let poll = cx.requester.send_poll(cx.update.chat.id, "Quel jour?", options, types::PollType::Regular);
	    let poll = poll.allows_multiple_answers(true);
	    let poll = poll.is_anonymous(false);
	    poll.await?;
	},
	Command::Ping => {cx.answer("pong!").await?;}
	
    };

    Ok(())
}

async fn run() -> serde_json::Result<()>{
    let mut file = File::open("./config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let v: serde_json::Value = serde_json::from_str(&data)?;
    
    teloxide::enable_logging!();

    let bot = Bot::new(v["token"].as_str().unwrap()).auto_send();

    let bot_name: String = "bot_name".to_string();
    bot.send_message(v["user"].as_i64().unwrap(), "I'm alive").await;
    teloxide::commands_repl(bot, bot_name, answer).await;
    
    Ok(())
}
