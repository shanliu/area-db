use log::warn;
use rusqlite::{Connection, Row};

use crate::{AreaCodeData, AreaDataProvider, AreaError, AreaGeoData, AreaGeoDataItem, AreaResult};

impl From<rusqlite::Error> for AreaError {
    fn from(err: rusqlite::Error) -> Self {
        AreaError::DB(err.to_string())
    }
}

pub struct SqliteAreaCodeData<'c, 't> {
    pub conn: &'c Connection,
    pub sql: &'t str,
    pub column_name: &'t str,
    pub column_code: &'t str,
    pub column_hide: &'t str,
    pub column_key_word: &'t str,
    pub key_word_name: bool,
}

impl<'c, 't> SqliteAreaCodeData<'c, 't> {
    pub fn from_conn(conn: &'c Connection) -> Self {
        Self {
            conn,
            sql: "select name,code,kw_name||','||kw_py as key_word,hide from area_code",
            column_name: "name",
            column_code: "code",
            column_hide: "hide",
            column_key_word: "key_word",
            key_word_name: true,
        }
    }
}

pub struct SqliteAreaGeoData<'c, 't> {
    pub conn: &'c Connection,
    pub sql: &'t str,
    pub column_code: &'t str,
    pub column_center: &'t str,
    pub column_polygon: &'t str,
}

impl<'c, 't> SqliteAreaGeoData<'c, 't> {
    pub fn from_conn(conn: &'c Connection) -> Self {
        Self {
            conn,
            sql: "select code,center,polygon from area_geo where code ='0' or code like '______%'",
            column_code: "code",
            column_center: "center",
            column_polygon: "polygon",
        }
    }
}

pub struct SqliteAreaData<'c, 't> {
    code_config: SqliteAreaCodeData<'c, 't>,
    geo_config: Option<SqliteAreaGeoData<'c, 't>>,
}
impl<'c, 't> SqliteAreaData<'c, 't> {
    pub fn new(
        code_config: SqliteAreaCodeData<'c, 't>,
        geo_config: Option<SqliteAreaGeoData<'c, 't>>,
    ) -> Self {
        Self {
            code_config,
            geo_config,
        }
    }
    fn read_data<T>(
        &self,
        conn: &Connection,
        sql: &str,
        f: impl for<'r> Fn(&'r Row<'r>) -> Option<T>,
    ) -> AreaResult<Vec<T>> {
        let mut stmt = conn.prepare(sql)?;
        let mut out = vec![];
        let mut rows = stmt.query(())?;
        while let Some(row) = rows.next()? {
            if let Some(item) = f(row) {
                out.push(item)
            }
        }
        Ok(out)
    }
}
impl<'c, 't> AreaDataProvider for SqliteAreaData<'c, 't> {
    fn read_code_data(&self) -> AreaResult<Vec<AreaCodeData>> {
        self.read_data(self.code_config.conn, self.code_config.sql, |row| {
            if let Ok(code) = row.get(self.code_config.column_code) {
                let hide = row.get(self.code_config.column_hide).unwrap_or(0);
                let name: Result<String, rusqlite::Error> = row.get(self.code_config.column_name);
                let keyword = row
                    .get(self.code_config.column_key_word)
                    .unwrap_or_else(|_| "".to_string());
                let mut key_word = keyword
                    .split(',')
                    .filter(|e| !e.is_empty())
                    .map(|e| e.to_owned())
                    .collect::<Vec<_>>();
                if self.code_config.key_word_name {
                    if let Ok(ename) = &name {
                        key_word.push(ename.to_owned());
                    }
                }
                Some(AreaCodeData {
                    code,
                    hide: hide == 1 || name.as_ref().map(|e| e.is_empty()).unwrap_or(true),
                    name: name.unwrap_or_else(|_| "[_._]".to_string()),
                    key_word,
                })
            } else {
                None
            }
        })
    }
    fn read_geo_data(&self) -> AreaResult<Vec<AreaGeoData>> {
        match &self.geo_config {
            Some(get_config) => {
                let out = self.read_data(get_config.conn, get_config.sql, |row| {
                    match row.get(get_config.column_code) {
                        Ok(code) => {
                            if let Ok(polygon) = row.get(get_config.column_polygon) {
                                return Some(AreaGeoData {
                                    code,
                                    item: vec![AreaGeoDataItem {
                                        center: row
                                            .get(get_config.column_center)
                                            .unwrap_or_else(|_| "".to_string()),
                                        polygon,
                                    }],
                                });
                            }
                        }
                        Err(e) => {
                            warn!("geo error:{}", e);
                            return None;
                        }
                    }
                    None
                })?;
                Ok(out)
            }
            None => Ok(vec![]),
        }
    }
}
