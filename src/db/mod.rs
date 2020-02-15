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

pub fn get_message(
    conn: &SqliteConnection,
    mid: u64,
) -> Result<models::Message, diesel::result::Error> {
    use self::schema::messages::dsl::*;

    let id_str = format!("{}", mid);
    let result = messages
        .filter(id.eq(&id_str))
        .limit(1)
        .load::<models::Message>(conn)?
        .into_iter()
        .nth(0)
        .unwrap();

    Ok(result)
}

pub fn test_and_save(msg: &Message) {
    if msg.author.bot {
        return;
    }

    if let Ok(url) = url::Url::parse(&msg.content) {
        let connection = make();
        if let Err(why) = save_message(
            &connection,
            format!("{}", msg.id).as_str(),
            format!("{}", msg.author.id).as_str(),
            format!("{}", url).as_str(),
        ) {
            log::error!("can't save message: {:?}", why);
        }
    }
}
