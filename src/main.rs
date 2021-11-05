use teloxide::prelude::*;

use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;
use std::fs;


#[tokio::main]
async fn main() -> Result<()> {
    let mut file = File::open("./config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let v: Value = serde_json::from_str(&data)?;
    
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::new(v["token"].as_str().unwrap()).auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer_dice().await?;
	log::info!("Replying");
        respond(())
    })
	.await;
    Ok(())
}
