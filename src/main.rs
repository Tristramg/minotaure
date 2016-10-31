#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate byteorder;
extern crate diesel_postgis;

use diesel::prelude::*;
use diesel_postgis::geo;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::nodes::dsl::*;
    use models::*;

    let conn = establish_connection();
    let results = nodes.load::<Node>(&conn).expect("Error loading nodes");
    // let results = <_ as LoadDsl<_>>::load(nodes, &conn).unwrap();
    for node in results {
        match node.geom {
            geo::Geometry::Point(p) => println!("x: {}, y: {}", p.x(), p.y()),
            _ => {}
        }
    }
}
