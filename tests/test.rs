#[cfg(feature = "data-mysql")]
#[test]
fn test_mysql() {
    let uri = "mysql://root:000@127.0.0.1:3306/test";
    let mysql = area_lib::MysqlAreaData::new(
        area_lib::MysqlAreaCodeData::from_conn(uri.to_string()),
        Some(area_lib::MysqlAreaGeoData::from_conn(uri.to_string())),
    );
    let area = area_lib::AreaDao::new(mysql).unwrap();
    test_branch(&area);
    let area = area.reload().unwrap();
    test_branch(&area);
}
#[cfg(any(feature = "data-sqlite", feature = "data-sqlite-source"))]
#[test]
fn test_sqlite() {
    use std::path::PathBuf;
    let uri = "data/area-data.db";
    let sqlite = area_lib::SqliteAreaData::new(
        area_lib::SqliteAreaCodeData::from_path(PathBuf::from(uri)),
        Some(area_lib::SqliteAreaGeoData::from_path(PathBuf::from(uri))),
    );
    let area = area_lib::AreaDao::new(sqlite).unwrap();
    test_branch(&area);
    let area = area.reload().unwrap();
    test_branch(&area);
}

// #[cfg(feature = "data-csv")]
// #[test]
// fn test_csv() {
//     use std::path::PathBuf;
//     let data = area_lib::CsvAreaData::new(
//         area_lib::CsvAreaCodeData::from_inner_path(PathBuf::from("data/2023-7-area-code.csv"))
//             .unwrap(),
//         Some(
//             area_lib::CsvAreaGeoData::from_inner_path(PathBuf::from("data/2023-7-area-geo.csv"))
//                 .unwrap(),
//         ),
//     );
//     test_branch(&area_lib::AreaDao::new(data).unwrap());
// }

#[cfg(all(feature = "data-csv-embed-geo", feature = "data-csv-embed-code"))]
#[test]
fn test_inner() {
    let data = area_lib::CsvAreaData::new(
        area_lib::CsvAreaCodeData::from_inner_data().unwrap(),
        Some(area_lib::CsvAreaGeoData::from_inner_data().unwrap()),
    );
    test_branch(&area_lib::AreaDao::new(data).unwrap());
}
#[allow(dead_code)]
fn test_branch(area: &area_lib::AreaDao) {
    for _ in 0..10 {
        let start = std::time::Instant::now();
        area.code_childs("441403").unwrap();
        let duration = start.elapsed();
        println!("code_childs is: {:?}", duration);
    }

    for _ in 0..10 {
        let start = std::time::Instant::now();
        area.code_find("130731").unwrap();
        let duration = start.elapsed();
        println!("code_find is: {:?}", duration);
    }

    for _ in 0..10 {
        let start = std::time::Instant::now();
        let res = area.code_search("广东 榕岗", 10).unwrap();
        let duration = start.elapsed();
        println!("{:?}", res[0]);
        println!("code_search is: {:?}", duration);
        let start = std::time::Instant::now();
        let res = area.code_search("guang dong", 10).unwrap();
        let duration = start.elapsed();
        println!("{:?}", res[0]);
        println!("code_search is: {:?}", duration);
    }
    for i in 0..10 {
        let start = std::time::Instant::now();
        area.geo_search(
            26.61474 + (i as f64 / 10000.0),
            114.13548 + (i as f64 / 10000.0),
        )
        .unwrap();
        let duration = start.elapsed();
        println!("geo_search is: {:?}", duration);
    }
}
