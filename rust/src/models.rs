use crate::schema::comments;

#[derive(Queryable)]
pub struct Comment {
    pub id: i32,
    pub author: String,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComment<'a> {
    pub author: &'a str,
    pub body: &'a str,
    pub created_at: chrono::NaiveDateTime,
}