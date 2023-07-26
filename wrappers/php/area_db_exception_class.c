/*
 * Author: shanliu  <shan.liu@msn.com>
 */
#include "zend.h"
#include "zend_API.h"
#include "zend_exceptions.h"
#include "php_area_db.h"
#include "area_db_exception_class.h"

static zend_function_entry area_db_exception_class_method[] = {
    ZEND_FE_END
};

zend_class_entry *area_db_exception_ce_ptr;
void area_db_exception_class_init(){
    zend_class_entry ce;
    INIT_NS_CLASS_ENTRY(ce,AREA_DB_NS,"Exception",area_db_exception_class_method);
    area_db_exception_ce_ptr = zend_register_internal_class_ex(&ce, zend_ce_exception);
}

