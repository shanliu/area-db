--TEST--
AreaDb test
--SKIPIF--
<?php
if (!extension_loaded('area_db')) {
    echo 'skip';
}
?>
--FILE--
<?php
use LsExt\AreaDb;
AreaDb::initCsv(__DIR__."/../../../data/2023-7-area-code.csv.gz",__DIR__."/../../../data/2023-7-area-geo.csv.gz");
$ret =AreaDb::codeSearch("guangdong");
echo count($ret);
$ret =AreaDb::codeSearch("ddddddddddddddddd");
echo count($ret);
$ret =AreaDb::codeFind("4414");
echo count($ret);
$ret =AreaDb::codeFind("121212121212");
echo count($ret);
$ret =AreaDb::codeRelated("4414");
echo count($ret);
$ret =AreaDb::codeRelated("121212121212");
echo count($ret);
$ret =AreaDb::geoSearch(22.57729, 113.89409);
echo count($ret);
$ret =AreaDb::codeChilds("");
echo count($ret);
$ret =AreaDb::codeChilds("1212121212121");
echo count($ret);
AreaDb::geoReload();
AreaDb::codeReload();
$ret =AreaDb::codeSearch("guangdong");
echo count($ret);
$ret =AreaDb::codeSearch("ddddddddddddddddd");
echo count($ret);
$ret =AreaDb::codeFind("4414");
echo count($ret);
$ret =AreaDb::codeFind("121212121212");
echo count($ret);
$ret =AreaDb::codeRelated("4414");
echo count($ret);
$ret =AreaDb::codeRelated("121212121212");
echo count($ret);
$ret =AreaDb::geoSearch(22.57729, 113.89409);
echo count($ret);
$ret =AreaDb::codeChilds("");
echo count($ret);
$ret =AreaDb::codeChilds("1212121212121");
echo count($ret);
AreaDb::shutdown();
?>
--EXPECT--
70203033407020303340
