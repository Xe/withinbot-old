use crate::mi;
use log::info;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};

#[command]
pub fn front(ctx: &mut Context, msg: &Message, _args: Args) -> CommandResult {
    let current_front: mi::switch::Switch = mi::client::CLIENT
        .get("https://mi.within.website/switches/current")
        .send()?
        .json()?;

    info!("front: {}", current_front.who);

    let response = MessageBuilder::new()
        .push("Current front: ")
        .push_bold(current_front.who)
        .build();

    if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
