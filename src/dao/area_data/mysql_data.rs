use mysql::{prelude::Queryable, Pool, Row};

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

impl From<mysql::Error> for AreaError {
    fn from(err: mysql::Error) -> Self {
        AreaError::DB(err.to_string())
    }
}

pub struct MysqlAreaCodeData<'t> {
    pub pool: Pool,
    pub sql: &'t str,
    pub column_name: &'t str,
    pub column_code: &'t str,
    pub column_hide: &'t str,
    pub column_key_word: &'t str,
    pub key_word_name: bool,
}

impl<'t> MysqlAreaCodeData<'t> {
    pub fn from_conn(pool: Pool) -> Self {
        Self {
            pool,
            sql: "select name,code,hide,key_word from area_code",
            column_name: "name",
            column_code: "code",
            column_hide: "hide",
            column_key_word: "key_word",
            key_word_name: true,
        }
    }
}

pub struct MysqlAreaGeoData<'t> {
    pub pool: Pool,
    pub sql: &'t str,
    pub column_code: &'t str,
    pub column_center: &'t str,
    pub column_polygon: &'t str,
}

impl<'t> MysqlAreaGeoData<'t> {
    pub fn from_conn(pool: Pool) -> Self {
        Self {
            pool,
            sql: "select code,center,polygon from area_geo where code in ('0') or code like '______%'",
            column_code: "code",
            column_center: "center",
            column_polygon: "polygon",
        }
    }
}

pub struct MysqlAreaData<'t> {
    code_config: MysqlAreaCodeData<'t>,
    geo_config: Option<MysqlAreaGeoData<'t>>,
}
impl<'t> MysqlAreaData<'t> {
    pub fn new(
        code_config: MysqlAreaCodeData<'t>,
        geo_config: Option<MysqlAreaGeoData<'t>>,
    ) -> Self {
        Self {
            code_config,
            geo_config,
        }
    }
}

impl<'t> AreaDataProvider for MysqlAreaData<'t> {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        let mut conn = self.code_config.pool.get_conn()?;
        let result: Vec<Row> = conn.query(self.code_config.sql).unwrap();
        Ok(result
            .into_iter()
            .flat_map(|row| {
                if let Some(code) = row.get(self.code_config.column_code) {
                    let hide = row.get(self.code_config.column_hide).unwrap_or(0);
                    let name: Option<String> = row.get(self.code_config.column_name);
                    let keyword: String = row
                        .get(self.code_config.column_key_word)
                        .unwrap_or_default();
                    let mut key_word = keyword
                        .split(',')
                        .filter(|e| !e.is_empty())
                        .map(|e| e.to_owned())
                        .collect::<Vec<_>>();
                    if self.code_config.key_word_name {
                        if let Some(ename) = &name {
                            key_word.push(ename.to_owned());
                        }
                    }
                    Some(AreaCodeData {
                        code,
                        hide: hide == 1 || name.as_ref().map(|e| e.is_empty()).unwrap_or(true),
                        name: name.unwrap_or_else(|| "[_._]".to_string()),
                        key_word,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>())
    }
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>> {
        match &self.geo_config {
            Some(get_config) => {
                let mut conn = get_config.pool.get_conn()?;
                let result: Vec<Row> = conn.query(get_config.sql).unwrap();
                Ok(result
                    .into_iter()
                    .flat_map(|row| {
                        if let Some(code) = row.get(get_config.column_code) {
                            if let Some(polygon) = row.get(get_config.column_polygon) {
                                return Some(AreaGeoData {
                                    code,
                                    item: vec![AreaGeoDataItem {
                                        center: row
                                            .get(get_config.column_center)
                                            .unwrap_or_default(),
                                        polygon,
                                    }],
                                });
                            }
                        }
                        None
                    })
                    .collect::<Vec<_>>())
            }
            None => Ok(vec![]),
        }
    }
}
