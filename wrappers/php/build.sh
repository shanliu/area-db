#!/bin/bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source ~/.profile
cd ../ && cargo build -r && cp ./target/release/area_db.h ./php_ext/lib && cp ./target/release/libarea_db.so ./php_ext/lib && cd php_ext
phpize && ./configure && make && make install
