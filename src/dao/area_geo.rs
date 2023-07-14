use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::{AreaError, AreaResult};
use geo::coordinate_position::{CoordPos, CoordinatePosition};
use geo::{
    coord, Centroid, Coord, GeodesicDistance, LineString, MinimumRotatedRect, Point, Polygon,
};
use rayon::prelude::*;
// 初始化

// OutlierDetection 坐标异常检测 :日志

// Centroid [省,市,县都预先算一遍],基于中心点构建B*TREE

// MinimumRotatedRect 最小边界框 [县都预先算一遍]

// CoordPos, CoordinatePosition 坐标是否在某区域检测

#[derive(Debug)]
struct AreaGeoSearchAreaIndex {
    rect: Polygon,
    detail: Polygon,
}

#[derive(Debug)]
struct AreaGeoSearchDataIndex {
    center: Point,
    area: AreaGeoSearchAreaIndex,
}
#[derive(Debug)]
struct AreaGeoSearchItemIndex {
    code: String,
    search_data: Vec<AreaGeoSearchDataIndex>,
}

#[derive(Debug)]
struct AreaGeoQueueItem<'t> {
    distance: u64,
    code: &'t str,
    search_data: &'t AreaGeoSearchAreaIndex,
}
impl<'t> PartialEq for AreaGeoQueueItem<'t> {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}
impl<'t> Eq for AreaGeoQueueItem<'t> {}
impl<'t> Ord for AreaGeoQueueItem<'t> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl<'t> PartialOrd for AreaGeoQueueItem<'t> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

pub struct AreaGeo {
    all_geo: Vec<AreaGeoSearchAreaIndex>,
    search_geo: Vec<AreaGeoSearchItemIndex>,
}

pub struct AreaGeoDataItem {
    pub center: String,
    pub polygon: String,
}
pub struct AreaGeoData {
    pub code: String,
    pub item: Vec<AreaGeoDataItem>,
}

impl AreaGeo {
    pub fn new(area_geo_data: &[AreaGeoData]) -> Self {
        let mut all_geo = Vec::with_capacity(area_geo_data.len());
        let mut search_geo = Vec::with_capacity(area_geo_data.len());
        for tmp_area in area_geo_data {
            let mut items = vec![];

            for tmp_item in tmp_area.item.iter() {
                let ps = tmp_item.polygon.split(',').collect::<Vec<_>>();
                let mut cs = Vec::with_capacity(ps.len());
                for pt in ps {
                    let mut iter = pt.split_whitespace();
                    if let (Some(x), Some(y)) = (iter.next(), iter.next()) {
                        if let Ok(x) = x.parse::<f64>() {
                            if let Ok(y) = y.parse::<f64>() {
                                cs.push(coord! { x:x, y:y});
                            }
                        }
                    }
                }
                if cs.is_empty() {
                    continue;
                }
                let detail = Polygon::new(cs.into_iter().collect::<LineString<f64>>(), vec![]);
                let rect = match MinimumRotatedRect::minimum_rotated_rect(&detail) {
                    Some(dat) => dat,
                    None => {
                        continue;
                    }
                };

                let mut iter = tmp_item.center.split_whitespace();
                let center = if let (Some(x), Some(y)) = (iter.next(), iter.next()) {
                    if let Ok(x) = x.parse::<f64>() {
                        if let Ok(y) = y.parse::<f64>() {
                            Some(coord! { x:x, y:y})
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                let center = if center.is_none() {
                    detail.centroid()
                } else {
                    center.map(|e| e.into())
                };
                let center = match center {
                    Some(e) => e,
                    None => continue,
                };
                items.push(AreaGeoSearchDataIndex {
                    center,
                    area: AreaGeoSearchAreaIndex {
                        rect,
                        detail: detail.clone(),
                    },
                });
            }
            if tmp_area.code == "0" || tmp_area.code.is_empty() {
                all_geo = items.into_iter().map(|e| e.area).collect::<Vec<_>>();
                continue;
            }
            search_geo.push(AreaGeoSearchItemIndex {
                code: tmp_area.code.to_owned(),
                search_data: items,
            });
        }
        Self {
            search_geo,
            all_geo,
        }
    }
    /// 通过坐标获取可能区域
    pub fn search(&self, coord: &Coord) -> AreaResult<&str> {
        if !self.all_geo.is_empty()
            && !self.all_geo.iter().any(|e| {
                if e.rect.coordinate_position(coord) == CoordPos::Inside {
                    return e.detail.coordinate_position(coord) == CoordPos::Inside;
                }
                false
            })
        {
            return Err(AreaError::NonFind(format!(
                "not china geo :{},{}",
                coord.x, coord.y
            )));
        }
        let point = std::convert::Into::<Point>::into(coord.to_owned());
        let num_cpus = num_cpus::get();
        let result = self
            .search_geo
            .par_chunks(self.search_geo.len() / num_cpus)
            .map(|tcs| {
                let mut out = Vec::with_capacity(tcs.len());
                for tc in tcs {
                    for tci in tc.search_data.iter() {
                        let distance = tci.center.geodesic_distance(&point).round() as u64;
                        if distance > 5000000 {
                            continue;
                        }
                        out.push(AreaGeoQueueItem {
                            distance,
                            search_data: &tci.area,
                            code: &tc.code,
                        })
                    }
                }
                out
            })
            .flat_map(|e| e)
            .collect::<Vec<_>>();
        let mut heap: BinaryHeap<AreaGeoQueueItem<'_>> = BinaryHeap::from(result);
        while let Some(max) = heap.pop() {
            if max.search_data.rect.coordinate_position(coord) == CoordPos::Inside {
                return Ok(max.code);
            }
        }
        Err(AreaError::NonFind(format!(
            "not any area :{},{}",
            coord.x, coord.y
        )))
    }
}
