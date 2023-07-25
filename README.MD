
### 5级行政区域数据

> Rust 实现 省市县镇村 数据查询 名称模糊搜索 及坐标对应城市搜索

> 提供[c的ffi接口](c_dome/main.c)，可以通过ffi转接到其他语言上【预计会弄个php扩展，封装个go包】

#### 包含以下数据：

1. 五级行政区域数据库: [sqlite数据](data/area-data-sqlite.7z) [csv数据](data/2023-7-area-code.csv.gz)

2. 县区级经纬度坐标数据 [sqlite数据](data/area-data-sqlite.7z) [csv数据](data/2023-7-area-geo.csv.gz)

3. 预计下一步引入：ip对应城市数据


#### 代码及功能说明：

> 可用于测试用命令 `cargo build` 会生成 `china-area` 命令

> 每次启动都构建一次内存索引，所以慢，仅测试用.

> 预计:准备做个gui界面，启动后多次查询。

```
#使用示例及输出
./target/release/china-area -s "广东 布吉"
开始构建索引
索引构建完成,开始查询
1:[440307014004]广东省-深圳市-龙岗区-布吉街道-德兴
2:[440307014002]广东省-深圳市-龙岗区-布吉街道-长龙
3:[440307014005]广东省-深圳市-龙岗区-布吉街道-龙珠
...省略显示...
```

##### 数据来源自定义

> 默认 features 功能

```toml
#cargo.toml
#默认的 features 启用 :
#data-csv-embed-code 省市县镇村 数据
#data-csv-embed-geo 县级坐标 数据
#lib-clib 输出c的动态库,用于其他语言对接
area-db = { version = "~0.0.*",git = "https://github.com/shanliu/area-db"}
```

> 如果不需要导出c的动态库，可以 使用 `default-features=false` 及不引入 `lib-clib` 来实现 


1. 仅使用内置压缩数据来源

> 默认使用当前crate的data目录数据的压缩数据，注意：未嵌入到二进制文件中。

> 数据未嵌入到二进制文件中，所以仅方便测试时使用，实际建议方法2代替。

```toml
#cargo.toml
area-db = { version = "~0.0.*", default-features=false,features=["data-csv-embed-code","data-csv-embed-geo"],git = "https://github.com/shanliu/area-db"}
```

```rust
let data = area_lib::CsvAreaData::new(
    area_lib::CsvAreaCodeData::from_inner_data().unwrap(),
    Some(area_lib::CsvAreaGeoData::from_inner_data().unwrap()),
);
let area = area_lib::AreaDao::new(data).unwrap();
```

2. 使用自定义的csv文件作为数据来源：[城市数据示例](data/2023-7-area-code.csv.gz) [城市坐标示例](data/2023-7-area-geo.csv.gz) 

> 仅开启csv文件支持，不引入内置数据源。好处:csv方便更新数据，不嵌入二进制中，二进制文件小

```toml
#cargo.toml
area-db = { version = "~0.0.*", default-features=false,features=["data-csv"],git = "https://github.com/shanliu/area-db"}
```

```rust
let data = area_lib::CsvAreaData::new(
    area_lib::CsvAreaCodeData::::from_inner_path(PathBuf::from("data/2023-7-area-code.csv.gz")).unwrap(),
    Some(area_lib::CsvAreaGeoData::from_inner_path(PathBuf::from("data/2023-7-area-geo.csv.gz")).unwrap()),
);
let area = area_lib::AreaDao::new(data).unwrap();
```

3. 使用sqlite作为数据来源：[表定义](data/sqlite-table.sql) [内置的包含5级行政区域跟坐标的sqlite数据](data/area-data-sqlite.7z) `解压后使用`

> 如果你已经装了lib-sqlite的库，可用:`data-sqlite` 会使用系统的sqlite库. 具体参见crate:`rusqlite`实现

> 如果你未安装lib-sqlite的库，可用:`data-sqlite-source` 会通过c源码编译sqlite库. sqlite源码通过子模块`git submodule update --init`获取

> 仅开启sqlite文件支持，不引入内置数据源，方便更新数据，不嵌入二进制中，二进制文件小

```toml
#cargo.toml
area-db = { version = "~0.0.*", default-features=false,features=["data-sqlite-source"],git = "https://github.com/shanliu/area-db"}
```

```rust
let area = area_lib::AreaDao::new(mysql).unwrap();
let conn = "data/area-data.db";
let sqlite = area_lib::SqliteAreaData::new(
    area_lib::SqliteAreaCodeData::from_path(PathBuf::from(&conn)),
    Some(area_lib::SqliteAreaGeoData::from_path(PathBuf::from(&conn))),
);
let area = area_lib::AreaDao::new(sqlite).unwrap();
```

4. 使用mysql作为数据来源：[表定义](data/mysql-table.sql) 数据可以把以上的csv数据导入进去

> 仅开启mysql数据库作为数据源的支持，以下声明不引入内置数据源,方便数据跟现有系统集成

```toml
#cargo.toml
area-db = { version = "~0.0.*", default-features=false,features=["data-mysql"],git = "https://github.com/shanliu/area-db"}
```

```rust
let pool = "mysql://***:***@127.~0.0.*:3306/***";
let mysql = area_lib::MysqlAreaData::new(
    area_lib::MysqlAreaCodeData::from_uri(pool),
    Some(area_lib::MysqlAreaGeoData::from_uri(pool)),
);
let area = area_lib::AreaDao::new(mysql).unwrap();
```



5. 数据更新时重新加载数据

```rust
let area = area_lib::AreaDao::new(...).unwrap();
area.geo_reload().unwrap();//重新加载GEO数据
area.code_reload().unwrap();//重新加载CODE数据
```


##### 可用的查询方法

1. 查询 省市县镇村 可用在地址选择多级联动接口

>  测试查询时间 100us 以内

```rust
let child="";//空列出省级，把省级的code转入列出市级..依次完成
let res = area.code_childs(child).unwrap();
println!("{}", &res);
```

2. 查询 指定code 的相关数据

```rust
let child="441403133";//地址code
let res = area.code_related(child).unwrap();
println!("{}", &res);
```

3. 查询 指定code 的详细地址信息

>  测试查询时间 100us 以内

> 可用于系统内的code到地址转换

```rust
let child="441403133";//地址code
let res = area.code_find(child).unwrap();
println!("{}", &res);
```

4. 搜索任意地址，返回匹配的地址信息

> 测试查询时间 50 ～ 80ms 

```rust
let child="guang dong";//地址信息 在比如: 广东 布吉
let limit = 10;//返回匹配数量
let res = area.code_search(child, limit).unwrap();
println!("{}", &res);
```

5. 返回指定坐标的地址信息

> 可用手机根据geo信息自动填写当前位置收货地址

> 测试查询时间 10 ～ 30ms 

> 目前到市一级，因为只找到了市一级的坐标数据

```rust
let res = area.geo_search(  26.61474,  114.13548 , ).unwrap();
println!("{}", &res);
```

6. 根据当前ip获取地址信息【待定，后面会补上】
