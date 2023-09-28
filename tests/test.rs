#[cfg(feature = "data-mysql")]
#[test]
fn test_mysql() {
    use area_db::AreaStoreMemory;
    let uri = "mysql://root:@127.0.0.1:3306/test";
    let mysql = area_db::MysqlAreaData::new(
        area_db::MysqlAreaCodeData::from_uri(uri, None),
        Some(area_db::MysqlAreaGeoData::from_uri(uri, None)),
    );
    let area = area_db::AreaDao::from_mysql_mem(mysql, AreaStoreMemory::default()).unwrap();
    test_branch(&area);
    area.geo_reload().unwrap();
    area.code_reload().unwrap();
    test_branch(&area);
}

#[cfg(any(feature = "data-sqlite", feature = "data-sqlite-source"))]
#[test]
fn test_sqlite() {
    use std::path::PathBuf;

    use area_db::AreaStoreMemory;
    let uri = "data/area-data.db";
    let sqlite = area_db::SqliteAreaData::new(
        area_db::SqliteAreaCodeData::from_path(PathBuf::from(uri)),
        Some(area_db::SqliteAreaGeoData::from_path(PathBuf::from(uri))),
    );
    let area = area_db::AreaDao::from_sqlite_mem(sqlite, AreaStoreMemory::default()).unwrap();
    test_branch(&area);
    area.geo_reload().unwrap();
    area.code_reload().unwrap();
    test_branch(&area);
}

#[cfg(feature = "data-csv")]
#[test]
fn test_csv() {
    use area_db::{AreaStoreDisk, AreaStoreMemory};

    let code_path = std::path::PathBuf::from(format!(
        "{}/data/2023-7-area-code.csv.gz",
        env!("CARGO_MANIFEST_DIR")
    ));
    let geo_data = {
        let geo_path = std::path::PathBuf::from(format!(
            "{}/data/2023-7-area-geo.csv.gz",
            env!("CARGO_MANIFEST_DIR")
        ));
        Some(area_db::CsvAreaGeoData::from_inner_path(geo_path, true).unwrap())
    };
    let data = area_db::CsvAreaData::new(
        area_db::CsvAreaCodeData::from_inner_path(code_path, true).unwrap(),
        geo_data,
    );
    test_branch(&area_db::AreaDao::from_csv_mem(data, AreaStoreMemory::default()).unwrap());
    // test_branch(
    //     &area_db::AreaDao::from_csv_disk(data, AreaStoreDisk::new("./tmp".into(), None)).unwrap(),
    // );
}

#[allow(dead_code)]
fn test_branch(area: &area_db::AreaDao) {
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
