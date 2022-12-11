// #[macro_use]
// extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use dotenv::dotenv;
use std::env;

pub fn connection() -> PgConnection {
    dotenv().ok();
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found!");
    let conn = PgConnection::establish(&db_url).expect("Cannot connect db");

    return conn;
}