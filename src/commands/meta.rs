use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    let _ = msg.channel_id.say(&ctx.http, "Pong!");

    Ok(())
}

#[command]
fn about(ctx: &mut Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(
        &ctx.http,
        "This is a first Rust project. See https://github.com/Xe/withinbot for more information.",
    ) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
