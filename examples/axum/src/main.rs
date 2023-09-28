use std::{collections::HashMap, path::PathBuf, sync::Arc};

use area::area_handler;
use area_db::{AreaDao, AreaStoreDisk, CsvAreaCodeData, CsvAreaData, CsvAreaGeoData};
use axum::{extract::Path, extract::Query, routing::get, Router};
use serde_json::json;
mod area;
#[tokio::main]
async fn main() {
    let mut index_dir = std::env::temp_dir();
    index_dir.push("area-db-data");
    let mut code_path = PathBuf::from("../../data/2023-7-area-code.csv.gz");
    if !code_path.is_file() {
        code_path = PathBuf::from("data/2023-7-area-code.csv.gz");
    }
    let mut geo_path = PathBuf::from("../../data/2023-7-area-geo.csv.gz");
    if !geo_path.is_file() {
        geo_path = PathBuf::from("data/2023-7-area-geo.csv.gz");
    }
    let data = CsvAreaData::new(
        CsvAreaCodeData::from_inner_path(code_path, true).unwrap(),
        CsvAreaGeoData::from_inner_path(geo_path, true).ok(),
    );
    let area_dao = Arc::new(
        AreaDao::from_csv_disk(data, AreaStoreDisk::new(index_dir, None).unwrap()).unwrap(),
    );
    let app = Router::new().route("/area/:path", {
        let area_dao = Arc::clone(&area_dao);
        get(
            |Path(path): Path<String>, Query(params): Query<HashMap<String, String>>| async {
                area_handler(path, params, area_dao)
                    .await
                    .unwrap_or_else(|e| {
                        json!({
                            "status":false,
                            "msg":e.to_string(),
                        })
                    })
                    .to_string()
            },
        )
    });
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
