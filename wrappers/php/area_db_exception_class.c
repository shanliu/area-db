/*
 * Author: shanliu  <shan.liu@msn.com>
 */
#include "zend.h"
#include "zend_API.h"
#include "zend_exceptions.h"
#include "php_area_db.h"
#include "area_db_exception_class.h"


zend_class_entry *area_db_exception_ce_ptr;
void area_db_exception_class_init(){
    zend_class_entry ce;
    INIT_NS_CLASS_ENTRY(ce,AREA_DB_NS,"Exception",class_LsExt_Exception_methods);
    area_db_exception_ce_ptr = zend_register_internal_class_ex(&ce, zend_ce_exception);
}

