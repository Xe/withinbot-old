use log::error;
use rand::prelude::*;
use serde_json::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

fn load_facts() -> Result<Vec<String>> {
    let data: &[u8] = include_bytes!("./printerfacts.json");
    let result: Vec<String> = serde_json::from_slice(data)?;

    Ok(result)
}

#[command]
pub fn printerfact(ctx: &mut Context, msg: &Message) -> CommandResult {
    let facts = load_facts()?;
    let i = thread_rng().gen::<usize>() % facts.len();

    if let Err(why) = msg.channel_id.say(&ctx.http, &facts[i]) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
