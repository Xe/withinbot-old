use log::error;
use rand::prelude::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

pub struct FactsContainer;

impl TypeMapKey for FactsContainer {
    type Value = pfacts::Facts;
}

pub fn make() -> pfacts::Facts {
    pfacts::make()
}

#[command]
pub fn printerfact(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let value = data.get::<FactsContainer>().unwrap();
    let ref facts = *value;
    let i = thread_rng().gen::<usize>() % facts.len();

    if let Err(why) = msg.channel_id.say(&ctx.http, &facts[i]) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
