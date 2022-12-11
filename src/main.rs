#[macro_use]
extern crate diesel;

#[path ="./database/conn.rs"]
mod conn;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use self::conn::*;

use self::models::*;
use self::schema::posts;
use self::schema::posts::dsl::*;

fn main() {
    let conn = connection();

    let newPost = NewPost {
        body: "test",
        slug: "test",
        title: "test"
    };
    
    let _post = diesel::insert_into(posts).values(&newPost).execute(&conn);

    let posts_result = posts.load::<Post>(&conn).expect("Error with posts");

    for post in posts_result {
        println!("{}", post.title);
    }

}
