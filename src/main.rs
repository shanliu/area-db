use clap::{App, Arg};
use std::process::exit;
fn main() {
    let mut app = App::new("China Area")
        .about("中国行政区域信息查询,仅用于测试")
        .arg(
            Arg::with_name("find")
                .short('f')
                .long("find")
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
    if matches.value_of("find").is_some()
        || matches.value_of("child").is_some()
        || matches.value_of("search").is_some()
        || matches.value_of("geo").is_some()
    {
        println!("开始构建索引");
    } else {
        app.print_help().unwrap();
        #[allow(clippy::needless_return)]
        return;
    }
    let data = area_lib::inner_csv_area_data(true).unwrap();
    let area = area_lib::AreaDao::new(data).unwrap_or_else(|e| output_error(e));
    println!("索引构建完成,开始查询");
    let matches = app.clone().get_matches();
    if let Some(code) = matches.value_of("find") {
        let res = area.code_find(code).unwrap_or_else(|e| output_error(e));
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
#[allow(dead_code)]
fn output_error(e: impl std::error::Error) -> ! {
    panic!("查询异常:{}", e);
    #[allow(unreachable_code)]
    exit(0)
}
#[allow(dead_code)]
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
