use teloxide::prelude::*;
use teloxide::types;


use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() -> Result<()>{
    let mut file = File::open("./config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let v: Value = serde_json::from_str(&data)?;
    
    teloxide::enable_logging!();

    let bot = Bot::new(v["token"].as_str().unwrap()).auto_send();

    teloxide::repl(bot, |message| async move {
        //message.answer_dice().await?;
	let msg = message.update;
	let res = match msg.kind {
	    types::MessageKind::Common(cm) => match cm.media_kind {
		types::MediaKind::Text(mkt) => mkt.text,
		_ => String::new()
	    }
	    _ => String::new()
	};
	log::info!("{}: {:?}", msg.date, res);
	let options = vec![
	    String::from("Lundi"),
	    String::from("Mardi"),
	    String::from("Mercredi"),
	    String::from("Jeudi"),
	    String::from("Vendredi"),
	    String::from("Samedi"),
	    String::from("Dimanche"),
	];
	let poll = message.requester.send_poll(msg.chat.id, "Quel jour?", options, types::PollType::Regular);
	let poll = poll.allows_multiple_answers(true);
	let poll = poll.is_anonymous(false);
	poll.await?;
	log::info!("Replying");
        respond(())
    })
	.await;
    Ok(())
}
