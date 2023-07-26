/* area_db extension for PHP */

#ifdef HAVE_CONFIG_H
# include "config.h"
#endif
#include "php.h"
#include "ext/standard/info.h"
#include "php_area_db.h"
#include "area_db_class.h"



/* {{{ PHP_MINIT_FUNCTION
 */
PHP_MINIT_FUNCTION(area_db)
{
	area_db_class_init();
	return SUCCESS;
}
/* }}} */

/* {{{ PHP_MINIT_FUNCTION
 */
PHP_MSHUTDOWN_FUNCTION(area_db)
{
    return SUCCESS;
}
/* }}} */

/* {{{ PHP_RINIT_FUNCTION */
PHP_RINIT_FUNCTION(area_db)
{
#if defined(ZTS) && defined(COMPILE_DL_AREA_DB)
	ZEND_TSRMLS_CACHE_UPDATE();
#endif

	return SUCCESS;
}
/* }}} */

/* {{{ PHP_MINFO_FUNCTION */
PHP_MINFO_FUNCTION(area_db)
{
	php_info_print_table_start();
	php_info_print_table_header(2, "area db support", "enabled");
	php_info_print_table_end();
}
/* }}} */

/* {{{ area_db_module_entry */
zend_module_entry area_db_module_entry = {
	STANDARD_MODULE_HEADER,
	"area_db",					/* Extension name */
	NULL,					/* zend_function_entry */
	PHP_MINIT(area_db),							/* PHP_MINIT - Module initialization */
    PHP_MSHUTDOWN(area_db),							/* PHP_MSHUTDOWN - Module shutdown */
	PHP_RINIT(area_db),			/* PHP_RINIT - Request initialization */
	NULL,							/* PHP_RSHUTDOWN - Request shutdown */
	PHP_MINFO(area_db),			/* PHP_MINFO - Module info */
	PHP_AREA_DB_VERSION,		/* Version */
	STANDARD_MODULE_PROPERTIES
};
/* }}} */

#ifdef COMPILE_DL_AREA_DB
# ifdef ZTS
ZEND_TSRMLS_CACHE_DEFINE()
# endif
ZEND_GET_MODULE(area_db)
#endif
