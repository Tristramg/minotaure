table!{
    nodes {
        id -> Integer,
        osm_id -> BigInt,
        geog -> ::diesel_postgis::Geometry,
    }
}
