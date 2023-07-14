-- 城市数据
CREATE TABLE "area_code" (
    "code" VARCHAR(12),
    "name" VARCHAR(128),
    "kw_name" VARCHAR(128),
    "kw_py" VARCHAR(256),
    "hide" INTEGER DEFAULT 0
) -- 坐标数据
CREATE TABLE "area_geo" (
    "code" VARCHAR(12),
    "center" VARCHAR(128),
    "polygon" TEXT
)