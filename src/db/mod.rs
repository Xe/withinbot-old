use diesel::{prelude::*, sqlite::SqliteConnection};
use serenity::model::channel::Message;
use std::env;

pub mod models;
pub mod schema;

pub fn make() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn save_message<'a>(
    conn: &SqliteConnection,
    id: &'a str,
    sender: &'a str,
    body: &'a str,
) -> Result<usize, diesel::result::Error> {
    use models::NewMessage;
    use schema::messages;

    let new_message = NewMessage {
        id: id,
        sender: sender,
        body: body,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(conn)
}

pub fn test_and_save(msg: &Message) {
    if !msg.content.starts_with("https://static1.e621.net/data/") {
        return;
    }

    match url::Url::parse(&msg.content) {
        Ok(url) => {
            let connection = make();
            match save_message(
                &connection,
                format!("{}", msg.id).as_str(),
                format!("{}", msg.author.id).as_str(),
                format!("{}", url).as_str(),
            ) {
                Ok(_) => {}
                Err(why) => {
                    log::error!("can't save message: {:?}", why);
                }
            }
        }
        Err(why) => {
            log::error!("can't parse url? {:?}", why);
        }
    }
}
