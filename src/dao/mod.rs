mod area_data;
pub use area_data::*;
mod area_code;
pub use area_code::*;
mod area_geo;
pub use area_geo::*;
use parking_lot::RwLock;

use std::{
    error::Error,
    fmt::{Display, Formatter},
};

//公共结构定义
#[derive(Debug)]
pub enum AreaError {
    DB(String),
    System(String),
    NotFind(String),
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
    fn code_data_is_change(&self) -> bool;
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>>;
    fn geo_data_is_change(&self) -> bool;
}

pub struct AreaDao {
    provider: Box<dyn AreaDataProvider + 'static + Sync + Send>,
    code: RwLock<AreaCode>,
    geo: RwLock<AreaGeo>,
}

impl AreaDao {
    pub fn new(provider: impl AreaDataProvider + 'static + Sync + Send) -> AreaResult<Self> {
        let tmp = provider.read_code_data()?;
        let code = AreaCode::new(&tmp);
        drop(tmp);
        let tmp = provider.read_geo_data()?;
        let geo = AreaGeo::new(&tmp);
        drop(tmp);
        Ok(Self {
            provider: Box::new(provider),
            code: RwLock::new(code),
            geo: RwLock::new(geo),
        })
    }
    pub fn code_reload(&self) -> AreaResult<()> {
        if self.provider.code_data_is_change() {
            let tmp = self.provider.read_code_data()?;
            let code = AreaCode::new(&tmp);
            drop(tmp);
            *self.code.write() = code;
        }
        Ok(())
    }
    pub fn geo_reload(&self) -> AreaResult<()> {
        if self.provider.geo_data_is_change() {
            let tmp = self.provider.read_geo_data()?;
            let geo = AreaGeo::new(&tmp);
            drop(tmp);
            *self.geo.write() = geo;
        }
        Ok(())
    }
    pub fn code_childs(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        self.code.read().childs(code).map(|mut e| {
            e.sort_by(|a, b| a.code.cmp(&b.code));
            e
        })
    }
    pub fn code_find(&self, code: &str) -> AreaResult<Vec<AreaCodeItem>> {
        self.code.read().find(code)
    }
    pub fn code_related(&self, code: &str) -> AreaResult<Vec<Vec<AreaCodeRelatedItem>>> {
        self.code.read().related(code).map(|e| {
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
            while let Ok(tmp) = self.code.read().childs("") {
                out.push(AreaSearchItem {
                    item: tmp,
                    key_word: "".to_string(),
                })
            }
            return Ok(out);
        }
        self.code.read().search(name, limit)
    }
    pub fn geo_search(&self, lat: f64, lng: f64) -> AreaResult<Vec<AreaCodeItem>> {
        let tmp = self.geo.read();
        let code = tmp.search(&geo::coord! { x:lng, y:lat})?;
        self.code.read().find(code)
    }
}

#[cfg(all(feature = "data-csv-embed-geo", feature = "data-csv-embed-code"))]
#[test]
fn test_code() {
    let data = crate::CsvAreaData::new(
        crate::CsvAreaCodeData::from_inner_data().unwrap(),
        Some(crate::CsvAreaGeoData::from_inner_data().unwrap()),
    );
    let area = crate::AreaDao::new(data).unwrap();
    let res = area.code_find("4414").unwrap();
    assert_eq!(res[1].code, "4414");
    let res = area.code_childs("").unwrap();
    assert!(res.iter().any(|e| e.code == "44"));
    let res = area.code_related("441403131203").unwrap();
    assert_eq!(res.len(), 5);
    let res = area.code_search("广东 梅州 南口", 10).unwrap();
    assert_eq!(res[0].item[1].code, "4414");
    let res = area.geo_search(22.57729, 113.89409).unwrap();
    assert_eq!(res[2].code, "440306");
}
