use crate::mi;
use chrono::prelude::*;
use log::info;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::ops::Sub;

#[command]
pub fn front(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let current_front: mi::switch::Switch = mi::client::CLIENT
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
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
