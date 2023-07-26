<?php

/** @generate-function-entries */

namespace LsExt{
    class Exception extends \Exception{
    }
    class AreaDb{
        /**
         * @throw Exception
         */
        static public function initCsv(string $code_path=NULL,string $geo_path=NULL):void{}
        /**
         * @throw Exception
         */
        static public function initSqlite(string $sqlite_sql):void{}
         /**
         * @throw Exception
         */
        static public function initMysql(string $uri):void{}
         /**
         * @throw Exception
         */
        static public function shutdown():void{}
         /**
         * @throw Exception
         */
        static public function geoReload():void{}
         /**
         * @throw Exception
         */
        static public function codeReload():void{}
         /**
         * @throw Exception
         */
        static public function codeChilds(string $code):array {}
         /**
         * @throw Exception
         */
        static public function codeSearch(string $code,int $limit):array {}
         /**
         * @throw Exception
         */
        static public function codeFind(string $code):array {}
         /**
         * @throw Exception
         */
        static public function codeRelated(string $code):array {}
         /**
         * @throw Exception
         */
        static public function geoSearch(float $lat,float $lng):array {}
    }
}