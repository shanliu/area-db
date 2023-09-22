/* This is a generated file, edit the .stub.php file instead.
 * Stub hash: d7443d1c29ce95c90974e5ef6755e4ab3c377bae */

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_initCsv, 0, 2, IS_VOID, 0)
	ZEND_ARG_TYPE_INFO(0, code_path, IS_STRING, 0)
	ZEND_ARG_TYPE_INFO(0, geo_path, IS_STRING, 0)
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_path, IS_STRING, 0, "NULL")
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_size, IS_LONG, 0, "0")
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, gz, _IS_BOOL, 0, "true")
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_initSqlite, 0, 1, IS_VOID, 0)
	ZEND_ARG_TYPE_INFO(0, sqlite_sql, IS_STRING, 0)
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_path, IS_STRING, 0, "NULL")
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_size, IS_LONG, 0, "0")
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_initMysql, 0, 1, IS_VOID, 0)
	ZEND_ARG_TYPE_INFO(0, uri, IS_STRING, 0)
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_path, IS_STRING, 0, "NULL")
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, index_size, IS_LONG, 0, "0")
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_shutdown, 0, 0, IS_VOID, 0)
ZEND_END_ARG_INFO()

#define arginfo_class_LsExt_AreaDb_geoReload arginfo_class_LsExt_AreaDb_shutdown

#define arginfo_class_LsExt_AreaDb_codeReload arginfo_class_LsExt_AreaDb_shutdown

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_codeChilds, 0, 1, IS_ARRAY, 0)
	ZEND_ARG_TYPE_INFO(0, code, IS_STRING, 0)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_codeSearch, 0, 1, IS_ARRAY, 0)
	ZEND_ARG_TYPE_INFO(0, code, IS_STRING, 0)
	ZEND_ARG_TYPE_INFO_WITH_DEFAULT_VALUE(0, limit, IS_LONG, 0, "10")
ZEND_END_ARG_INFO()

#define arginfo_class_LsExt_AreaDb_codeFind arginfo_class_LsExt_AreaDb_codeChilds

#define arginfo_class_LsExt_AreaDb_codeRelated arginfo_class_LsExt_AreaDb_codeChilds

ZEND_BEGIN_ARG_WITH_RETURN_TYPE_INFO_EX(arginfo_class_LsExt_AreaDb_geoSearch, 0, 2, IS_ARRAY, 0)
	ZEND_ARG_TYPE_INFO(0, lat, IS_DOUBLE, 0)
	ZEND_ARG_TYPE_INFO(0, lng, IS_DOUBLE, 0)
ZEND_END_ARG_INFO()


ZEND_METHOD(LsExt_AreaDb, initCsv);
ZEND_METHOD(LsExt_AreaDb, initSqlite);
ZEND_METHOD(LsExt_AreaDb, initMysql);
ZEND_METHOD(LsExt_AreaDb, shutdown);
ZEND_METHOD(LsExt_AreaDb, geoReload);
ZEND_METHOD(LsExt_AreaDb, codeReload);
ZEND_METHOD(LsExt_AreaDb, codeChilds);
ZEND_METHOD(LsExt_AreaDb, codeSearch);
ZEND_METHOD(LsExt_AreaDb, codeFind);
ZEND_METHOD(LsExt_AreaDb, codeRelated);
ZEND_METHOD(LsExt_AreaDb, geoSearch);


static const zend_function_entry class_LsExt_Exception_methods[] = {
	ZEND_FE_END
};


static const zend_function_entry class_LsExt_AreaDb_methods[] = {
	ZEND_ME(LsExt_AreaDb, initCsv, arginfo_class_LsExt_AreaDb_initCsv, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, initSqlite, arginfo_class_LsExt_AreaDb_initSqlite, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, initMysql, arginfo_class_LsExt_AreaDb_initMysql, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, shutdown, arginfo_class_LsExt_AreaDb_shutdown, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, geoReload, arginfo_class_LsExt_AreaDb_geoReload, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, codeReload, arginfo_class_LsExt_AreaDb_codeReload, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, codeChilds, arginfo_class_LsExt_AreaDb_codeChilds, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, codeSearch, arginfo_class_LsExt_AreaDb_codeSearch, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, codeFind, arginfo_class_LsExt_AreaDb_codeFind, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, codeRelated, arginfo_class_LsExt_AreaDb_codeRelated, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(LsExt_AreaDb, geoSearch, arginfo_class_LsExt_AreaDb_geoSearch, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_FE_END
};
