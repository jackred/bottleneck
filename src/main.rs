use teloxide::prelude::*;
use teloxide::types::{MessageKind, MediaKind, User};

extern crate chrono;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};

use serde_json::{Result, Value};
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    run().await;
}

fn unwrap_or_default<T, U>(to_unwrap: Option<T>, default: U, unwrap_fn: &dyn Fn(T)-> U) -> U {
    match to_unwrap {
	Some(res) => unwrap_fn(res),
	None => default
    }
}

fn unwrap_user(user: User) -> String{
    let mut res = String::from(user.first_name);
    if let Some(name) = user.last_name {
	res += " ";
	res += &name;
    }
    if let Some(name) = user.username {
	res += " (";
	res += &name;
	res += ")";
    }
    res
}


fn unwrap_sender(user: Option<User>) -> String{
    unwrap_or_default(user, "channel message".to_string(), &unwrap_user)
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
	let mut res = String::new();
	match msg.kind {
	    MessageKind::Common(mk) => {
		res += "Common Message";
		res = format!("{} from {}", res, unwrap_sender(mk.from));
		match mk.media_kind {
		    MediaKind::Text(mkt) => {
			res = format!("{}{}{}", res, "\n", mkt.text);
		    },
		    _ => {}
		}},
	    MessageKind::NewChatMembers(mk) => {
		res += "Member new to the chat";
	    },
	    MessageKind::LeftChatMember(mk) => {
		res += "Member left the chat";
	    },
	    MessageKind::NewChatTitle(mk) => {
		res += "New chat title";
	    },
	    MessageKind::NewChatPhoto(mk) => {
		res += "New chat photo";
	    },
	    MessageKind::DeleteChatPhoto(mk) => {
		res += "Chat photo deleted";
	    },
	    MessageKind::GroupChatCreated(mk) => {
		res += "New group chat";
	    },
	    MessageKind::SupergroupChatCreated(mk) => {
		res += "New super group chat";
	    },
	    MessageKind::ChannelChatCreated(mk) => {
		res += "New channel created";
	    },
	    MessageKind::MessageAutoDeleteTimerChanged(mk) => {
		res += "Timer delete message changed";
	    },
	    MessageKind::Migrate(mk) => {
		res += "Migrated?";
	    },
	    MessageKind::Pinned(mk) => {
		res += "Pinned";
	    },
	    MessageKind::Invoice(mk) => {
		res += "Invoice?";
	    },
	    MessageKind::SuccessfulPayment(mk) => {
		res += "Succesful payment?";
	    },
	    MessageKind::ConnectedWebsite(mk) => {
		res += "Connected website?";
	    },
	    MessageKind::PassportData(mk) => {
		res += "Passport data?";
	    },
	    MessageKind::Dice(mk) => {
		res += "Dice?";
	    },
	    MessageKind::ProximityAlertTriggered(mk) => {
		res += "Proximity alert triggered?";
	    },
	    MessageKind::VoiceChatScheduled(mk) => {
		res += "Voice chat scheduled";
	    },
	    MessageKind::VoiceChatStarted(mk) => {
		res += "Voice chat started";
	    },
	    MessageKind::VoiceChatEnded(mk) => {
		res += "Voice chat ended";
	    },
	    MessageKind::VoiceChatParticipantsInvited(mk) => {
		res += "Voice chat participant invited";
	    },
	    _ => {}
	};
	log::info!("{}: {:?}", NaiveDateTime::from_timestamp(msg.date.into(), 0), "Message received!");
	println!("{}", res);
	// let options = vec![
	//     String::from("Lundi"),
	//     String::from("Mardi"),
	//     String::from("Mercredi"),
	//     String::from("Jeudi"),
	//     String::from("Vendredi"),
	//     String::from("Samedi"),
	//     String::from("Dimanche"),
	// ];
	// let poll = message.requester.send_poll(msg.chat.id, "Quel jour?", options, types::PollType::Regular);
	// let poll = poll.allows_multiple_answers(true);
	// let poll = poll.is_anonymous(false);
	// poll.await?;
	log::info!("Replying");
        respond(())
    })
	.await;
    Ok(())
}
