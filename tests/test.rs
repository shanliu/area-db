// #[test]
// fn test_mysql() {
//     let pool = mysql::Pool::new("mysql://root:000@127.0.0.1:3306/test").unwrap();
//     let mysql = area_lib::MysqlAreaData::new(
//         area_lib::MysqlAreaCodeData::from_conn(pool.clone()),
//         Some(area_lib::MysqlAreaGeoData::from_conn(pool)),
//     );
//     let area = area_lib::AreaDao::new(mysql).unwrap();
//     test_branch(area)
// }
// #[test]
// fn test_sqlite() {
//     let conn = rusqlite::Connection::open("data/area-data.db").unwrap();
//     let sqlite = area_lib::SqliteAreaData::new(
//         area_lib::SqliteAreaCodeData::from_conn(&conn),
//         Some(area_lib::SqliteAreaGeoData::from_conn(&conn)),
//     );
//     let area = area_lib::AreaDao::new(sqlite).unwrap();
//     drop(conn);
//     test_branch(area)
// }
// #[test]
// fn test_csv() {
//     let data = area_lib::CsvAreaData::new(
//         area_lib::CsvAreaCodeData::path("data/2023-7-area-code.csv").unwrap(),
//         Some(area_lib::CsvAreaGeoData::path("data/2023-7-area-geo.csv").unwrap()),
//     );
//     test_branch(area_lib::AreaDao::new(data).unwrap())
// }
#[test]
fn test_inner() {
    let data = area_lib::CsvAreaData::new(
        area_lib::CsvAreaCodeData::inner_data().unwrap(),
        Some(area_lib::CsvAreaGeoData::inner_data().unwrap()),
    );
    test_branch(area_lib::AreaDao::new(data).unwrap())
}
fn test_branch(area: area_lib::AreaDao) {
    for _ in 0..10 {
        let start = std::time::Instant::now();
        area.code_childs("441403133").unwrap();
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
