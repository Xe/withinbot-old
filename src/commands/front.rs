use reqwest::blocking::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::env;

#[command]
pub fn front(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let front_url = env::var("FRONT_URL").expect("wanted FRONT_URL");
    let front = get(&front_url)?.text()?;

    let response = MessageBuilder::new()
        .push("Current front:")
        .push_bold(front)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
