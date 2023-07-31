/*
 * Author: shanliu  <shan.liu@msn.com>
 */

#ifndef PHP_AERA_DB_CLASS_H
#define PHP_AERA_DB_CLASS_H
#if PHP_VERSION_ID < 80000
#include "area_db_arginfo_7.h"
#else
#include "area_db_arginfo.h"
#endif

extern zend_class_entry *area_db_ce_ptr;
void area_db_class_init();
#endif





