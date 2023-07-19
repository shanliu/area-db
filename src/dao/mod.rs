mod area_data;
pub use area_data::*;
mod area_code;
pub use area_code::*;
mod area_geo;
pub use area_geo::*;

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

//公共结构定义
#[derive(Debug)]
pub enum AreaError {
    DB(String),
    System(String),
    NonFind(String),
}
impl Display for AreaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for AreaError {}
pub type AreaResult<T> = Result<T, AreaError>;

pub trait AreaDataProvider {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>>;
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>>;
}

pub struct AreaDao {
    code: AreaCode,
    geo: AreaGeo,
}

impl AreaDao {
    pub fn new(data: impl AreaDataProvider) -> AreaResult<Self> {
        let area_code_data = data.read_code_data()?;
        let area_geo_data = data.read_geo_data()?;
        Ok(Self {
            code: AreaCode::new(&area_code_data),
            geo: AreaGeo::new(&area_geo_data),
        })
    }
    pub fn code_childs(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        self.code.childs(code).map(|mut e| {
            e.sort_by(|a, b| a.code.cmp(&b.code));
            e
        })
    }
    pub fn code_find(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        self.code.find(code)
    }
    pub fn code_detail(&self, code: &str) -> AreaResult<Vec<Vec<AreaCodeDetailItem>>> {
        self.code.detail(code).map(|e| {
            e.into_iter()
                .map(|mut ie| {
                    ie.sort_by(|a, b| a.item.code.cmp(&b.item.code));
                    ie
                })
                .collect::<Vec<_>>()
        })
    }
    pub fn code_search(&self, name: &str, limit: usize) -> AreaResult<Vec<AreaSearchItem>> {
        if name.trim().is_empty() {
            let mut out = Vec::with_capacity(limit);
            while let Ok(tmp) = self.code.childs("") {
                out.push(AreaSearchItem {
                    item: tmp,
                    key_word: "".to_string(),
                })
            }
            return Ok(out);
        }
        self.code.search(name, limit)
    }
    pub fn geo_search(&self, lat: f64, lng: f64) -> AreaResult<Vec<AreaCodeItem>> {
        let code = self.geo.search(&geo::coord! { x:lng, y:lat})?;
        self.code.find(code)
    }
}

#[test]
fn test_code() {
    let data = crate::CsvAreaData::new(
        crate::CsvAreaCodeData::inner_data().unwrap(),
        Some(crate::CsvAreaGeoData::inner_data().unwrap()),
    );
    let area = crate::AreaDao::new(data).unwrap();
    let res = area.code_find("4414").unwrap();
    assert_eq!(res[1].code, "4414");
    let res = area.code_childs("").unwrap();
    assert!(res.iter().any(|e| e.code == "44"));
    let res = area.code_detail("441403131203").unwrap();
    assert_eq!(res.len(), 5);
    let res = area.code_search("广东 梅州 南口", 10).unwrap();
    assert_eq!(res[0].item[1].code, "4414");
    let res = area.geo_search(22.57729, 113.89409).unwrap();
    assert_eq!(res[2].code, "440306");
}
