/*
#[cfg(feature = "data-mysql")]
#[test]
fn test_mysql() {
    let uri = "mysql://root:000@127.0.0.1:3306/test";
    let mysql = area_lib::MysqlAreaData::new(
        area_lib::MysqlAreaCodeData::from_uri(uri),
        Some(area_lib::MysqlAreaGeoData::from_uri(uri)),
    );
    let area = area_lib::AreaDao::new(mysql).unwrap();
    test_branch(&area);
    area.geo_reload().unwrap();
    area.code_reload().unwrap();
    test_branch(&area);
}
*/
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
    area.geo_reload().unwrap();
    area.code_reload().unwrap();
    test_branch(&area);
}

#[cfg(feature = "data-csv")]
#[test]
fn test_csv() {
    let data = area_lib::inner_csv_area_data(true).unwrap();
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
