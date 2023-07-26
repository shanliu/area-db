/* area_db extension for PHP */

#ifndef PHP_AREA_DB_H
# define PHP_AREA_DB_H

extern zend_module_entry area_db_module_entry;
# define phpext_area_db_ptr &area_db_module_entry

# define PHP_AREA_DB_VERSION "0.0.1"

# if defined(ZTS) && defined(COMPILE_DL_AREA_DB)
ZEND_TSRMLS_CACHE_EXTERN()
# endif



#include "area_db.h"


ZEND_BEGIN_MODULE_GLOBALS(area_db)
        CAreaDao* area_dao;
                ZEND_END_MODULE_GLOBALS(area_db)

ZEND_DECLARE_MODULE_GLOBALS(area_db)


#define AREA_DB_G(v) ZEND_MODULE_GLOBALS_ACCESSOR(area_db, v)


#define AREA_DB_NS  "LsExt"




#endif	/* PHP_AREA_DB_H */
