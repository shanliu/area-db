/*
 * Author: shanliu  <shan.liu@msn.com>
 */

#include <stdlib.h>
#include "zend.h"
#include "zend_API.h"
#include "zend_exceptions.h"
#include "area_db_class.h"
#include "php_area_db.h"
#include "area_db_exception_class.h"
#include "area_db.h"

CAreaDao* area_dao;
#define GET_CAREA_DAO() area_dao

#ifdef PHP_WIN32
#include <windows.h>
extern SRWLOCK lock;
#define CAREA_R_LOCK()  AcquireSRWLockShared(&lock);
#define CAREA_R_UNLOCK() ReleaseSRWLockShared(&lock);
#define CAREA_W_LOCK() AcquireSRWLockExclusive(&lock);
#define CAREA_W_UNLOCK()  ReleaseSRWLockExclusive(&lock);
#else
#include <pthread.h>
extern pthread_rwlock_t lock;
#define CAREA_R_LOCK() pthread_rwlock_rdlock(&lock);
#define CAREA_R_UNLOCK()  pthread_rwlock_unlock(&lock);
#define CAREA_W_LOCK() pthread_rwlock_wrlock(&lock);
#define CAREA_W_UNLOCK() pthread_rwlock_unlock(&lock);
#endif




void ret_item_vec(struct CAreaItemVec *out_data, zval *return_value){
    array_init(return_value);
    u_long len2 = out_data->len;
    CAreaItem *tmp2 = out_data->data;
    while (len2-- > 0) {
        zval tmp_item;
        array_init(&tmp_item);
        add_assoc_string(&tmp_item, "name", tmp2->name);
        add_assoc_string(&tmp_item, "code", tmp2->code);
        add_assoc_bool(&tmp_item, "leaf", tmp2->leaf);
        add_next_index_zval(return_value, &tmp_item);
        tmp2++;
    }
    area_db_release_item_vec(out_data);
}




void throw_area_exception(const zend_long code,const char* fmt,char* ret_err){
    zend_throw_exception_ex(area_db_exception_ce_ptr, code,fmt,ret_err);
    area_db_release_error_str(ret_err);
}


ZEND_METHOD(AreaDb, initCsv){
    char *code_file;
    size_t code_file_len=0;
    char *geo_file;
    size_t geo_file_len=0;
    zend_bool gz=0;

#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "pp|b", &code_file, &code_file_len, &geo_file, &geo_file_len,&gz) == FAILURE) {
        return;
    }
#else
    ZEND_PARSE_PARAMETERS_START(2, 3)
        Z_PARAM_PATH(code_file, code_file_len)
        Z_PARAM_PATH(geo_file, geo_file_len)
        Z_PARAM_OPTIONAL
        Z_PARAM_BOOL(gz)
    ZEND_PARSE_PARAMETERS_END();
#endif
    char *ret_err=NULL;
    int ret_no=0;
    unsigned char sgz=gz==0?0:1;
CAREA_W_LOCK()
    if (GET_CAREA_DAO() == NULL) {
        ret_no=area_db_init_csv(code_file,geo_file,&sgz,&GET_CAREA_DAO(),&ret_err);
    }
CAREA_W_UNLOCK()
    if (ret_no!=0){
        throw_area_exception(ret_no,"init csv data fail:%s",ret_err);
    }
}

ZEND_METHOD(AreaDb, initSqlite){
    char *filename;
    size_t filename_len;
#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "p", &filename, &filename_len) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 1)
        Z_PARAM_PATH(filename, filename_len)
    ZEND_PARSE_PARAMETERS_END();
#endif

#if HAVE_AREA_DB_USE_SQLITE
    char *ret_err=NULL;
    int ret_no=0;
    CAREA_W_LOCK()
    if (GET_CAREA_DAO()  == NULL) {
        ret_no=area_db_init_sqlite(filename,&GET_CAREA_DAO() ,&ret_err);
    }
    CAREA_W_UNLOCK()
    if (ret_no!=0){
        throw_area_exception(ret_no,"init sqlite fail:%s",ret_err);
    }
#else
    zend_throw_exception_ex(area_db_exception_ce_ptr, 400,"not support sqlite");
#endif
    RETURN_NULL();
}


ZEND_METHOD(AreaDb, initMysql){
    char *uri;
    size_t uri_len;
#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "p", &uri, &uri_len) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 1)
        Z_PARAM_PATH(uri, uri_len)
    ZEND_PARSE_PARAMETERS_END();
#endif
#if HAVE_AREA_DB_USE_MYSQL
    char *ret_err=NULL;
    int ret_no=0;
    if (GET_CAREA_DAO()  == NULL) {
        ret_no=area_db_init_mysql(uri,&GET_CAREA_DAO() ,&ret_err);
    }
    if (ret_no!=0){
        throw_area_exception(ret_no,"init mysql fail:%s",ret_err);
    }
#else
    zend_throw_exception_ex(area_db_exception_ce_ptr, 400,"not support MYSQL");
#endif
    RETURN_NULL();
}

ZEND_METHOD(AreaDb, shutdown){
CAREA_W_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        area_db_release_area_dao(GET_CAREA_DAO() );
        GET_CAREA_DAO()=NULL;
    }
CAREA_W_UNLOCK()
}



ZEND_METHOD(AreaDb, codeReload){
    char *ret_err=NULL;
    int ret_no=0;

    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_code_reload(&GET_CAREA_DAO() ,&ret_err);
    }

    if (ret_no!=0){
        throw_area_exception(ret_no,"reload code fail:%s",ret_err);
    }
}
ZEND_METHOD(AreaDb, geoReload){
    char *ret_err=NULL;
    int ret_no=0;
CAREA_W_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_geo_reload(&GET_CAREA_DAO() ,&ret_err);
    }
CAREA_W_UNLOCK()
    if (ret_no!=0){
        throw_area_exception(ret_no,"reload geo fail:%s",ret_err);
    }
}



ZEND_METHOD(AreaDb, codeChilds){
    char *code=NULL;
    size_t code_len = 0;

#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &code, &code_len) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 1)
        Z_PARAM_STRING(code, code_len)
    ZEND_PARSE_PARAMETERS_END();
#endif
    CAreaItemVec* area_vec2=NULL;
    char *ret_err=NULL;
    int ret_no=0;
CAREA_R_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_code_childs(code,GET_CAREA_DAO() ,&area_vec2,&ret_err);
    }
CAREA_R_UNLOCK()
    if(ret_no!=0){
        throw_area_exception(ret_no,"child parse fail:%s",ret_err);
    }else {
        ret_item_vec(area_vec2,return_value);
    }
}


ZEND_METHOD(AreaDb, codeFind){
    char *code=NULL;
    size_t code_len = 0;
#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &code, &code_len) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 1)
        Z_PARAM_STRING(code, code_len)
    ZEND_PARSE_PARAMETERS_END();
#endif
    CAreaItemVec* area_vec2=NULL;
    char *ret_err=NULL;
    int ret_no=0;
CAREA_R_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_code_find(code,GET_CAREA_DAO() ,&area_vec2,&ret_err);
    }
CAREA_R_UNLOCK()
    if(ret_no!=0){
        throw_area_exception(ret_no,"child find fail:%s",ret_err);
    }else {
        ret_item_vec(area_vec2,return_value);
    }
}


ZEND_METHOD(AreaDb, codeSearch){
    char *code=NULL;
    size_t code_len = 0;
    zend_long limit=10;
#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "s|l", &code, &code_len,&limit) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 2)
        Z_PARAM_STRING(code, code_len)
        Z_PARAM_OPTIONAL
        Z_PARAM_LONG(limit)
    ZEND_PARSE_PARAMETERS_END();
#endif
    CAreaItemVecs* area_vec1=NULL;
    char *ret_err=NULL;
    int ret_no=0;
CAREA_R_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_code_search(code,limit<=0?10:limit,GET_CAREA_DAO() ,&area_vec1,&ret_err);
    }
CAREA_R_UNLOCK()
    if(ret_no!=0){
        throw_area_exception(ret_no,"search fail:%s",ret_err);
    }else {
        array_init(return_value);
        u_long len1 = area_vec1->len;
        CAreaItemVec *tmp1 = area_vec1->data;
        while (len1-- > 0) {
            zval tmp_item1;
            array_init(&tmp_item1);
            u_long len2 = tmp1->len;
            CAreaItem *tmp2 = tmp1->data;
            while (len2-- > 0) {
                zval tmp_item;
                array_init(&tmp_item);
                add_assoc_string(&tmp_item, "name", tmp2->name);
                add_assoc_string(&tmp_item, "code", tmp2->code);
                add_assoc_bool(&tmp_item, "leaf", tmp2->leaf);
                add_next_index_zval(&tmp_item1, &tmp_item);
                tmp2++;
            }
            add_next_index_zval(return_value, &tmp_item1);
            tmp1++;
        }
        area_db_release_item_vecs(area_vec1);
    }
}

ZEND_METHOD(AreaDb, codeRelated){
    char *code=NULL;
    size_t code_len = 0;

#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "s", &code, &code_len) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(1, 2)
            Z_PARAM_STRING(code, code_len)

    ZEND_PARSE_PARAMETERS_END();
#endif
    CAreaRelatedItemVecs * area_vec1=NULL;
    char *ret_err=NULL;
    int ret_no=0;
CAREA_R_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no=area_db_code_related(code,GET_CAREA_DAO() ,&area_vec1,&ret_err);
    }
CAREA_R_UNLOCK()
    if(ret_no!=0){
        throw_area_exception(ret_no,"related get fail:%s",ret_err);
    }else {
        array_init(return_value);
        u_long len1 = area_vec1->len;
        CAreaRelatedItemVec *tmp1 = area_vec1->data;
        while (len1-- > 0) {
            zval tmp_item1;
            array_init(&tmp_item1);
            u_long len2 = tmp1->len;
            CAreaRelatedItem *tmp2 = tmp1->data;
            while (len2-- > 0) {
                zval tmp_item;
                array_init(&tmp_item);
                add_assoc_string(&tmp_item, "name", tmp2->name);
                add_assoc_string(&tmp_item, "code", tmp2->code);
                add_assoc_bool(&tmp_item, "leaf", tmp2->leaf);
                add_assoc_bool(&tmp_item, "selected", tmp2->selected);
                add_next_index_zval(&tmp_item1, &tmp_item);
                tmp2++;
            }
            add_next_index_zval(return_value, &tmp_item1);
            tmp1++;
        }
        area_db_release_related_vecs(area_vec1);
    }
}

ZEND_METHOD(AreaDb, geoSearch){
    double lat=0.0;
    double lng=0.0;
#if PHP_VERSION_ID < 80000
    if (zend_parse_parameters(ZEND_NUM_ARGS(), "dd", &lat, &lng) == FAILURE) {
            return;
        }
#else
    ZEND_PARSE_PARAMETERS_START(2, 2)
            Z_PARAM_DOUBLE(lat)
            Z_PARAM_DOUBLE(lng)
    ZEND_PARSE_PARAMETERS_END();
#endif
    CAreaItemVec* area_vec1=NULL;
    char *ret_err=NULL;
    int ret_no=0;
CAREA_R_LOCK()
    if (GET_CAREA_DAO()  != NULL) {
        ret_no= area_db_geo_search(lat,lng,GET_CAREA_DAO() ,&area_vec1,&ret_err);
    }
CAREA_R_UNLOCK()
    if(ret_no!=0){
        throw_area_exception(ret_no,"geo get fail:%s",ret_err);
    }else {
        ret_item_vec(area_vec1,return_value);
    }
}

zend_class_entry *area_core_ce_ptr;
void area_db_class_init() {
    zend_class_entry ce;
    INIT_NS_CLASS_ENTRY(ce, AREA_DB_NS, "AreaDB", class_AreaDb_methods);
    area_core_ce_ptr = zend_register_internal_class(&ce);
    zend_declare_property_null(area_core_ce_ptr, ZEND_STRL("res"), ZEND_ACC_PRIVATE);
}

