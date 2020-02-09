use crate::mi::{client::ClientContainer, switch::Switch};
use chrono::prelude::*;
use log::error;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::ops::Sub;
use reqwest::blocking::Client;

#[command]
pub fn front(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let data = ctx.data.read();
    let value = data.get::<ClientContainer>().unwrap();
    let ref cli: Client = *value.lock().unwrap();
    let current_front: Switch = cli
        .get("https://mi.within.website/switches/current")
        .send()?
        .json()?;

    let now = Utc::now();
    let dur = now.sub(current_front.started_at.unwrap());
    let response = MessageBuilder::new()
        .push("Current front: ")
        .push_bold(current_front.who)
        .push(" for ")
        .push(format!(
            "{} hours, {} minutes",
            dur.num_hours(),
            dur.num_minutes() % 60,
        ))
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
