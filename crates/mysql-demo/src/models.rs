use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub content: String,
    pub is_deleted: bool,
}

#[derive(Queryable)]
pub struct AuthGroup {
    pub id: i32,
    pub name: String,
}
