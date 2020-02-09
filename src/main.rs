mod commands;

use log::{error, info};
use serenity::{
    client::bridge::gateway::ShardManager,
    framework::{standard::macros::group, StandardFramework},
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use std::{collections::HashSet, env, sync::Arc};

use commands::{e621::*, front::*, meta::*, owner::*};
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
}

#[group]
#[commands(about, ping, quit)]
struct General;

#[group]
#[commands(front)]
struct Within;

#[group]
#[commands(get_post, search)]
struct E621;

fn main() {
    let _ = kankyo::load();
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
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
            .configure(|c| {
                c.owners(owners)
                    .prefix("~")
                    .on_mention(Some(bot_id))
                    .no_dm_prefix(true)
            })
            .help(&commands::help::MY_HELP)
            .normal_message(|ctx, msg| {
                commands::e621::resolve_links(ctx, msg);
            })
            .group(&GENERAL_GROUP)
            .group(&WITHIN_GROUP)
            .group(&E621_GROUP),
    );

    if let Err(why) = client.start() {
        error!("Client error: {:?}", why);
    }
}
