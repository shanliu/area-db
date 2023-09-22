-- 城市数据
CREATE TABLE `area_code` (
    `id` int(11) NOT NULL AUTO_INCREMENT,
    `name` varchar(32) DEFAULT NULL,
    `code` varchar(12) DEFAULT NULL,
    `hide` int(11) DEFAULT 0,
    `kw_name` varchar(128) DEFAULT NULL,
    `kw_py` varchar(128) DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB AUTO_INCREMENT = 83045 DEFAULT CHARSET = utf8mb4;
-- 坐标数据
CREATE TABLE `area_geo` (
    `id` varchar(100) DEFAULT NULL,
    `polygon` longtext DEFAULT NULL,
    `geo` varchar(100) DEFAULT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB AUTO_INCREMENT = 3246 DEFAULT CHARSET = utf8mb4;