use log::{error, info};
use serde::*;
use serenity::{
    framework::standard::{
        macros::{check, command},
        Args, CheckResult,
        CheckResult::{Failure, Success},
        CommandOptions, CommandResult,
        Reason::User,
    },
    model::prelude::*,
    prelude::*,
    utils::MessageBuilder,
};
use std::sync::{Arc, Mutex};
use url::Url;
use xe621::client::Client;

pub static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (by Adorcable on e621 +https://github.com/Xe/withinbot)"
);

pub struct ClientContainer;

impl TypeMapKey for ClientContainer {
    type Value = Arc<Mutex<Client>>;
}

pub fn make() -> Arc<Mutex<Client>> {
    Arc::new(Mutex::new(Client::new(APP_USER_AGENT).unwrap()))
}

#[check]
#[name = "nsfw_only"]
fn nsfw_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(chan) = msg.channel(&ctx.cache) {
        if chan.is_nsfw() {
            return Success;
        }
    } else {
        if let Ok(chan) = ctx.http.get_channel(*msg.channel_id.as_u64()) {
            if chan.is_nsfw() {
                return Success;
            }
        }
    }

    Failure(User("run this in a NSFW channel".to_string()))
}

#[command]
#[checks(nsfw_only)]
pub fn search(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read();
    let value = data.get::<ClientContainer>().unwrap();
    let ref cli: Client = *value.lock().unwrap();
    let tags = args.raw().collect::<Vec<&str>>();

    info!("{} searches for {:?}", msg.author.name, tags);

    let mut response = MessageBuilder::new();

    for post in cli.post_search(&tags[..]).take(3) {
        let mut p = post?;
        response.push(format!(
            "{} - {} - {:?} - {:?} {}\n",
            p.id,
            p.rating,
            p.artists,
            p.tags.truncate(10),
            p.file_url.unwrap()
        ));
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &response.build()) {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[checks(nsfw_only)]
pub fn get_post(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read();
    let value = data.get::<ClientContainer>().unwrap();
    let ref cli: Client = *value.lock().unwrap();
    let id = args.single::<u64>()?;

    info!("{} getting post {}", msg.author.name, id);

    match cli.get_post(id) {
        Ok(p) => {
            let response = MessageBuilder::new()
                .push(format!("{} - {} - {:?} - ", p.id, p.rating, p.artists,))
                .push(p.file_url.unwrap())
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response) {
                error!("Error sending message: {:?}", why);
            }

            let mut response = MessageBuilder::new();
            response.push_line("Tags:");

            for (i, tag) in p.tags.iter().enumerate() {
                response.push_safe(tag);

                if i + 1 != p.tags.len() {
                    response.push(", ");
                }
            }

            if let Err(why) = msg.channel_id.say(&ctx.http, &response.build()) {
                error!("Error sending message: {:?}", why);
            }
        }
        Err(e) => {
            error!("error fetching post: {:?}", e);
        }
    }

    Ok(())
}

pub fn resolve_link(ctx: Context, link: String) -> Option<String> {
    let data = ctx.data.read();
    let value = data.get::<ClientContainer>().unwrap();
    let ref cli: Client = *value.lock().unwrap();

    if !link.starts_with("https://static1.e621.net/data/") {
        return None;
    }

    log::debug!("resolving {}", link);
    let url = Url::parse(&link).unwrap();
    let path = std::path::Path::new(url.path());
    let md5 = path.file_stem().unwrap().to_str().unwrap();
    let body = cli.get_json_endpoint(&format!("/post/check_md5.json?md5={}", md5));
    let md5c: MD5Check = serde_json::from_value(body.unwrap()).unwrap();
    Some(MessageBuilder::new()
        .push(format!("<https://e621.net/post/show/{}>", md5c.post_id))
        .build())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MD5Check {
    pub md5: String,
    pub exists: bool,
    pub post_id: u64,
}
