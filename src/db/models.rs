use super::schema::messages;

#[derive(Queryable)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name="messages"]
pub struct NewMessage<'a> {
    pub id: &'a str,
    pub sender: &'a str,
    pub body: &'a str,
}
