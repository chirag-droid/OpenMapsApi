use poem::{web::Data, Result};
use poem_openapi::{param::Query, payload::Json, Object, OpenApi};

type DbPool = sqlx::PgPool;

pub struct OpenMapsApi;

/// Human readable address
#[derive(Object, sqlx::FromRow)]
struct GeoAddress {
    id: Option<i64>,
    name: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
    district: Option<String>,
}

fn default_precision() -> f64 {
    400.0
}
fn example_latitude() -> f64 {
    29.363133
}
fn example_longitude() -> f64 {
    75.898217
}

#[OpenApi(prefix_path = "/api")]
impl OpenMapsApi {
    /// Reverse Geocode
    ///
    /// ## Summary
    ///
    /// Returns a human readable address of a coordinate.
    #[oai(path = "/reverse", method = "get")]
    async fn reverse(
        &self,
        pool: Data<&DbPool>,
        #[oai(example = "example_latitude")] latitude: Query<f64>,
        #[oai(example = "example_longitude")] longitude: Query<f64>,
        #[oai(default = "default_precision", example = "default_precision")] precision: Query<f64>,
    ) -> Result<Json<Vec<GeoAddress>>> {
        let result = sqlx::query_as!(GeoAddress, "
            --Since our osm data is in 3857 coordinate system we transform from 4326 system to 3857
            --4326 is commonly used geodetic system
            --For geodetic coordinates, X is longitude and Y is latitude
            WITH gps_point AS (
                SELECT ST_Transform(ST_SetSRID(ST_MakePoint($1, $2), 4326), 3857) AS point
            )
            SELECT
                point.osm_id as id,
                COALESCE (point.tags->'name:en', point.name) AS name,
                ST_Y(ST_Transform(point.way, 4326)) AS latitude,
                ST_X(ST_Transform(point.way, 4326)) AS longitude,
                COALESCE(point.tags->'addr:full', point.tags->'addr:housename') AS address,
                COALESCE(point.tags->'addr:city', point.tags->'addr:hamlet', point.tags->'addr:village') AS city,
                point.tags->'addr:state' AS state,
                COALESCE(point.tags->'addr:district', point.tags->'addr:suburb', point.tags->'addr:neighbourhood') AS district
            FROM
                planet_osm_point as point, gps_point
            WHERE
                ST_DWithin(point.way, gps_point.point, $3)
            ORDER BY
                point.way <-> gps_point.point
        ", longitude.0, latitude.0, precision.0).fetch_all(pool.0).await.unwrap();

        Ok(Json(result))
    }
}
