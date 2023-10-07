<?php

//1.配置数据文件路径
const CODE_DATA_PATH = __DIR__ . "/../../data/2023-7-area-code.csv.gz";
const GEO_DATA_PATH = __DIR__ . "/../../data/2023-7-area-geo.csv.gz";
//2. 测试命令
//php -S localhost:8000
//curl 'http://localhost:8000/area_db.api.php?code=4414'
//curl 'http://localhost:8000/area_db.api.php?action=code&code=4414'
//curl 'http://localhost:8000/area_db.api.php?key_word=guangdong&action=search'
//curl 'http://localhost:8000/area_db.api.php?action=related&code=4414'
//curl 'http://localhost:8000/area_db.api.php?action=geo&lat=26.61474&lng=114.13548'
use LsExt\AreaDb;

try {
    AreaDb::initCsv(CODE_DATA_PATH, GEO_DATA_PATH, sys_get_temp_dir()."/area_db");
    $out = array('status' => true, 'msg' => 'ok', 'data' => action($_GET));
} catch (Exception $e) {
    $out = array('status' => FALSE, 'msg' => $e->getMessage());
}
echo json_encode($out, JSON_UNESCAPED_UNICODE);

//--------------------- action -------------------------------
function action(array $param)
{
    switch ($param['action'] ?? '') {
        case 'list':
        case '':
            return AreaDb::codeChilds($param['code'] ?? '');
        case 'search':
            if (!empty($param['key_word'])) {
                return AreaDb::codeSearch($param['key_word']);
            } else {
                return AreaDb::codeChilds("");
            }
        case 'code':
            return AreaDb::codeFind($param['code'] ?? '');
        case 'geo':
            return AreaDb::geoSearch(floatval($param['lat'] ?? '0'), floatval($param['lng'] ?? '0'));
        case 'related':
            return AreaDb::codeRelated($param['code'] ?? '');
        case 'reload':
            AreaDb::geoReload();
            AreaDb::codeReload();
            return [];
        default:
            throw new ErrorException("bad action");
    }
}
