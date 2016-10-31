use diesel_postgis::*;

// #[derive(Queryable)] doesn’t seem to work
pub struct Node {
    pub id: i32,
    pub osm_id: i64,
    pub geom: Geom,
}

Queryable! {
    pub struct Node {
        pub id: i32,
        pub osm_id: i64,
        pub geom: Geom,
    }
}
