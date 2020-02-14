use log::error;
use rand::prelude::*;
use serde_json::*;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::sync::{Arc, Mutex};

pub struct FactsContainer;

impl TypeMapKey for FactsContainer {
    type Value = Arc<Mutex<Vec<String>>>;
}

fn load_facts() -> Result<Vec<String>> {
    let data: &[u8] = include_bytes!("./printerfacts.json");
    let result: Vec<String> = serde_json::from_slice(data)?;

    Ok(result)
}

pub fn make() -> Arc<Mutex<Vec<String>>> {
    let facts = load_facts().unwrap(); // could panic, i guess

    Arc::new(Mutex::new(facts))
}

#[command]
pub fn printerfact(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();
    let value = data.get::<FactsContainer>().unwrap();
    let ref facts = *value.lock()?;
    let i = thread_rng().gen::<usize>() % facts.len();

    if let Err(why) = msg.channel_id.say(&ctx.http, &facts[i]) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
