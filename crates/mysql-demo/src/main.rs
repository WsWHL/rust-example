mod models;
mod schema;

use self::models::*;
use self::schema::article::dsl::*;
use self::schema::auth_group::dsl::*;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::Connection;
use dotenv::dotenv;
use std::env;

fn get_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", database_url);

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = &mut get_connection();

    let groups: Vec<AuthGroup> = auth_group
        .limit(5)
        .load::<AuthGroup>(connection)
        .expect("Error loading auth_group");
    println!("Displaying {} groups", groups.len());
    for group in groups {
        println!("id: {}, name: {}", group.id, group.name);
    }

    let articles: Vec<Article> = article
        .filter(title.eq("test"))
        .load::<Article>(connection)
        .expect("Error loading article");
    for item in articles {
        println!("{:?}", item)
    }
}
