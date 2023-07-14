use std::process::exit;

use clap::{App, Arg};
fn main() {
    let mut app = App::new("China Area")
        .about("中国行政区域信息查询,仅用于测试")
        .arg(
            Arg::with_name("code")
                .short('d')
                .long("code")
                .takes_value(true)
                .help("查看指定城市编码信息"),
        )
        .arg(
            Arg::with_name("child")
                .short('c')
                .long("child")
                .takes_value(true)
                .help("查看指定城市编码的下一级"),
        )
        .arg(
            Arg::with_name("geo")
                .short('g')
                .long("geo")
                .takes_value(true)
                .help("查看指定经纬度的城市,格式示例:22.573603,114.210972"),
        )
        .arg(
            Arg::with_name("search")
                .short('s')
                .long("search")
                .takes_value(true)
                .help("搜索城市"),
        );
    let matches = app.clone().get_matches();
    if matches.value_of("code").is_some()
        || matches.value_of("child").is_some()
        || matches.value_of("search").is_some()
        || matches.value_of("geo").is_some()
    {
        println!("开始构建索引");
    } else {
        app.print_help().unwrap();
        return;
    }
    // let pool = mysql::Pool::new("mysql://root:000@127.0.0.1:3306/test").unwrap();
    // let mysql = area_lib::MysqlAreaData::new(
    //     area_lib::MysqlAreaCodeData::from_conn(pool.clone()),
    //     Some(area_lib::MysqlAreaGeoData::from_conn(pool)),
    // );
    // let area = area_lib::AreaDao::new(mysql).unwrap();
    // let conn = rusqlite::Connection::open("data/area-data.db").unwrap();
    // let sqlite = area_lib::SqliteAreaData::new(
    //     area_lib::SqliteAreaCodeData::from_conn(&conn),
    //     Some(area_lib::SqliteAreaGeoData::from_conn(&conn)),
    // );
    // let area = area_lib::AreaDao::new(sqlite).unwrap();
    // drop(conn);
    // let data = area_lib::CsvAreaData::new(
    //     area_lib::CsvAreaCodeData::path("data/2023-7-area-code.csv").unwrap(),
    //     Some(area_lib::CsvAreaGeoData::path("data/2023-7-area-geo.csv").unwrap()),
    // );
    let data = area_lib::CsvAreaData::new(
        area_lib::CsvAreaCodeData::inner_data().unwrap(),
        Some(area_lib::CsvAreaGeoData::inner_data().unwrap()),
    );
    let area = area_lib::AreaDao::new(data).unwrap_or_else(|e| output_error(e));

    println!("索引构建完成,开始查询");
    let matches = app.clone().get_matches();
    if let Some(code) = matches.value_of("code") {
        let res = area.code_detail(code).unwrap_or_else(|e| output_error(e));
        println!("{}", format_item(&res));
    } else if let Some(child) = matches.value_of("child") {
        let res = area.code_childs(child).unwrap_or_else(|e| output_error(e));
        println!("{}", format_item(&res));
    } else if let Some(search) = matches.value_of("search") {
        let res = area
            .code_search(search, 10)
            .unwrap_or_else(|e| output_error(e));
        for (i, t) in res.iter().enumerate() {
            println!("{}:{}", i + 1, format_item(&t.item));
        }
    } else if let Some(geo) = matches.value_of("geo") {
        let coords: Vec<&str> = geo.split(',').collect();
        let lat: f64 = coords[0].parse().expect("lat not a float");
        let lng: f64 = coords[1].parse().expect("lng not a float");
        let res = area
            .geo_search(lat, lng)
            .unwrap_or_else(|e| output_error(e));
        println!("{}", format_item(&res));
    }
}
fn output_error(e: impl std::error::Error) -> ! {
    panic!("查询异常:{}", e);
    #[allow(unreachable_code)]
    exit(0)
}

fn format_item(item: &[area_lib::AreaCodeItem]) -> String {
    let mut strout = "".to_string();
    let mut code = "".to_string();
    for (i, tmp) in item.iter().enumerate() {
        if i + 1 != item.len() {
            strout += &format!("{}-", tmp.name);
        } else {
            code = format!("[{}]", tmp.code);
            strout += &tmp.name;
        }
    }
    format!("{}{}", code, strout)
}

// #[test]
// fn test_branch() {
//     let data = area_lib::CsvAreaData::new(
//         area_lib::CsvAreaCodeData::inner_data().unwrap(),
//         Some(area_lib::CsvAreaGeoData::inner_data().unwrap()),
//     );
//     let area = area_lib::AreaDao::new(data).unwrap_or_else(|e| output_error(e));

//     for _ in 0..1 {
//         let start = std::time::Instant::now();
//         area.code_childs("441403133").unwrap();
//         let duration = start.elapsed();
//         println!("code_childs is: {:?}", duration);
//     }

//     for _ in 0..1 {
//         let start = std::time::Instant::now();
//         area.code_detail("130731").unwrap();
//         let duration = start.elapsed();
//         println!("code_detail is: {:?}", duration);
//     }

//     for _ in 0..1000 {
//         let start = std::time::Instant::now();
//         let res = area.code_search("广东 榕岗", 10).unwrap();
//         let duration = start.elapsed();
//         println!("{:?}", res[0]);
//         println!("code_search is: {:?}", duration);
//         let start = std::time::Instant::now();
//         let res = area.code_search("guang dong", 10).unwrap();
//         let duration = start.elapsed();
//         println!("{:?}", res[0]);
//         println!("code_search is: {:?}", duration);
//     }
//     for i in 0..1000 {
//         let start = std::time::Instant::now();
//         area.geo_search(
//             114.13548 + (i as f64 / 10000.0),
//             26.61474 + (i as f64 / 10000.0),
//         )
//         .unwrap();
//         let duration = start.elapsed();
//         println!("geo_search is: {:?}", duration);
//     }
// }
