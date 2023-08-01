
#### 编译前准备动态库

> 兼容PHP>=7.0

```
#安装cargo 参考: https://www.rustup.rs
cd ../ 
#修改 cargo.toml 先启用 features=["lib-clib"]
cargo build -r && ./php_ext
cp ./target/release/area_db.h ./php_ext/lib/area_db.h
#windows:
cp ./target/release/libarea_db.so ./php_ext/lib/libarea_db.so
#linux or mac
cp ./target/release/area_db.dll ./php_ext/lib/area_db.dll
cp ./target/release/area_db.dll.lib ./php_ext/lib/area_db.lib

```


#### linux or mac 下编译

> 可选编译参数

```
--with-area_db_dir=./lib 自定义上面编译的so文件路径
--with-area_db_use_mysql 启用mysql数据源支持,需开启cargo.toml中的 features data-mysql 
--with-area_db_use_sqlite 启用sqlite数据源支持,需开启cargo.toml中的 features data-sqlite
```

```shell
phpize && configure
make && make install
```

#### windows 下编译

> [php 8.1 windows dll](https://github.com/shanliu/area_db/releases)

> load build env

```
打开 [x64|x86] Native Tools Command Prompt for VS 2017 编译命令行
加载PHP编译变量 \your-php-sdk-binary-tools-dir\bin\phpsdk_setvars.bat 
```

> 可选编译参数
```
--enable-area_db_use_mysql 启用mysql数据源支持,需开启cargo.toml中的 features data-mysql 
--enable-area_db_use_sqlite 启用sqlite数据源支持,需开启cargo.toml中的 features data-sqlite
```

```cmd
cd your_php_src # PHP 源码目录 下载 simhash 源码到 ext/simhash 下
buildconf.bat
configure --disable-all --enable-cli --enable-area_db=shared
nmake
```

> 使用时拷贝 area_db.dll 到 php.exe 的目录
