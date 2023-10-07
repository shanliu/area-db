/* area_db extension for PHP */

#ifndef PHP_AREA_DB_H
# define PHP_AREA_DB_H

extern zend_module_entry area_db_module_entry;
# define phpext_area_db_ptr &area_db_module_entry

# define PHP_AREA_DB_VERSION "0.1.1"

# if defined(ZTS) && defined(COMPILE_DL_AREA_DB)
ZEND_TSRMLS_CACHE_EXTERN()
# endif

#define AREA_DB_NS  "LsExt"
#include "php_version.h"
#if (PHP_VERSION_ID >= 80000)
#include "area_db_arginfo.h"
#else
#include "area_db_arginfo_7.h"
#endif



#ifdef PHP_WIN32
#include <windows.h>
SRWLOCK lock;
#else
#include <pthread.h>
pthread_rwlock_t lock;
#endif






#endif	/* PHP_AREA_DB_H */
