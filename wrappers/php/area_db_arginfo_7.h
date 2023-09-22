//read image class

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_reload_arginfo, 0, 0, 0)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(area_db_code_childs_arginfo, 0, 0, 1)
ZEND_ARG_INFO(0, code)
ZEND_END_ARG_INFO()


ZEND_BEGIN_ARG_INFO_EX(area_db_code_detail_arginfo, 0, 0, 1)
ZEND_ARG_INFO(0, code)
ZEND_END_ARG_INFO()


ZEND_BEGIN_ARG_INFO_EX(area_db_code_search_arginfo, 0, 0, 1)
ZEND_ARG_INFO(0, code)
ZEND_ARG_INFO(0, limit)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(area_geo_search_arginfo, 0, 0, 2)
ZEND_ARG_INFO(0, lat)
ZEND_ARG_INFO(0, lng)
ZEND_END_ARG_INFO()




ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_initCsv, 0,0, 2)
	ZEND_ARG_INFO(1,code_path)
	ZEND_ARG_INFO(1, geo_path)
	ZEND_ARG_INFO(0, index_path)
	ZEND_ARG_INFO(0, index_size)
	ZEND_ARG_INFO(0, gz)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_initSqlite, 0, 0,1)
	ZEND_ARG_INFO(1, sqlite_sql)
	ZEND_ARG_INFO(0, index_path)
	ZEND_ARG_INFO(0, index_size)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_initMysql, 0, 0,1)
	ZEND_ARG_INFO(1, uri)
	ZEND_ARG_INFO(0, index_path)
	ZEND_ARG_INFO(0, index_size)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_shutdown, 0, 0, 0)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_codeChilds, 0, 0,  1)
	ZEND_ARG_INFO(0, code)
ZEND_END_ARG_INFO()

ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_codeDetail, 0, 0,  1)
ZEND_ARG_INFO(0, code)
ZEND_END_ARG_INFO()


ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_codeFind, 0, 0,  1)
ZEND_ARG_INFO(0, code)
ZEND_END_ARG_INFO()


ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_codeSearch, 0, 0,  1)
	ZEND_ARG_INFO(1, code)
	ZEND_ARG_INFO(0, limit)
ZEND_END_ARG_INFO()


ZEND_BEGIN_ARG_INFO_EX(arginfo_class_AreaDb_geoSearch, 0,0, 2)
	ZEND_ARG_INFO(1, lat)
	ZEND_ARG_INFO(1, lng)
ZEND_END_ARG_INFO()


ZEND_METHOD(AreaDb, initCsv);
ZEND_METHOD(AreaDb, initSqlite);
ZEND_METHOD(AreaDb, initMysql);
ZEND_METHOD(AreaDb, shutdown);
ZEND_METHOD(AreaDb, codeReload);
ZEND_METHOD(AreaDb, geoReload);
ZEND_METHOD(AreaDb, codeChilds);
ZEND_METHOD(AreaDb, codeSearch);
ZEND_METHOD(AreaDb, codeFind);
ZEND_METHOD(AreaDb, codeRelated);
ZEND_METHOD(AreaDb, geoSearch);

static const zend_function_entry class_AreaDb_methods[] = {
	ZEND_ME(AreaDb, initCsv, arginfo_class_AreaDb_initCsv, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, initSqlite, arginfo_class_AreaDb_initSqlite, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, initMysql, arginfo_class_AreaDb_initMysql, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, shutdown, arginfo_class_AreaDb_shutdown, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, geoReload, arginfo_class_AreaDb_reload_arginfo, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, codeReload, arginfo_class_AreaDb_reload_arginfo, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, codeChilds, arginfo_class_AreaDb_codeChilds, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, codeSearch, arginfo_class_AreaDb_codeSearch, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, codeFind, arginfo_class_AreaDb_codeFind, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, codeRelated, arginfo_class_AreaDb_codeDetail, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_ME(AreaDb, geoSearch, arginfo_class_AreaDb_geoSearch, ZEND_ACC_PUBLIC|ZEND_ACC_STATIC)
	ZEND_FE_END
};
