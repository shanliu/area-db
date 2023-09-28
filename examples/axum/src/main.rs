use std::{collections::HashMap, path::PathBuf, sync::Arc};

use area::area_handler;
use area_db::{AreaDao, AreaStoreDisk, CsvAreaCodeData, CsvAreaData, CsvAreaGeoData};
use axum::{extract::Path, extract::Query, routing::get, Router};
use serde_json::json;
mod area;
use temp_dir::TempDir;
#[tokio::main]
async fn main() {
    let binding = TempDir::new().unwrap();
    let index_dir = binding.path().to_path_buf();
    let code_path = PathBuf::from("../../data/2023-7-area-code.csv.gz");
    let geo_path = PathBuf::from("../../data/2023-7-area-geo.csv.gz");
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
