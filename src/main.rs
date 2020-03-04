mod commands;

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{channel::Reaction, event::ResumedEvent, gateway::Ready, prelude::ReactionType::*},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

#[macro_use]
extern crate diesel;

use commands::{e621::*, front::*, meta::*, owner::*, printerfacts::*};
mod db;
mod mi;

struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }

    fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let connection = db::make();
        match reaction.emoji {
            Unicode(emoji) => match emoji.as_str() {
                "🔍" => {
                    info!(
                        "sauce lookup on {}/{}",
                        reaction.channel_id, reaction.message_id
                    );
                    match db::get_message(&connection, *reaction.message_id.as_u64()) {
                        Ok(msg) => {
                            let response = commands::e621::resolve_link(&ctx, msg.body);

                            if let Err(why) = reaction.channel_id.say(&ctx.http, response.unwrap())
                            {
                                error!("Error sending message: {:?}", why);
                            }
                        }

                        Err(why) => {
                            error!("can't get message from sqlite: {:?}", why);
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

#[group]
#[commands(about, ping, printerfact, quit)]
struct General;

#[group]
#[commands(front)]
struct Within;

#[group]
#[commands(get_post, search)]
struct E621;

fn main() {
    let _ = kankyo::init();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ClientContainer>(commands::e621::make());
        data.insert::<mi::client::ClientContainer>(mi::client::make());
        data.insert::<FactsContainer>(commands::printerfacts::make());
    }

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            (set, info.id)
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).on_mention(Some(bot_id)).no_dm_prefix(true))
            .help(&commands::help::MY_HELP)
            .normal_message(|ctx, msg| {
                db::test_and_save(msg);
            })
            .unrecognised_command(|ctx, msg, name| {
                let response = &format!("can't find command {}", name);
                if let Err(why) = msg.channel_id.say(&ctx.http, response) {
                    error!("Error sending message: {:?}", why);
                }
            })
            .on_dispatch_error(|_, _, why| {
                error!("dispatch error: {:?}", why);
            })
            .group(&GENERAL_GROUP)
            .group(&WITHIN_GROUP)
            .group(&E621_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
