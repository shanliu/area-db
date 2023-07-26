#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct AreaDao AreaDao;

typedef struct CAreaDao {
  struct AreaDao *dao;
} CAreaDao;

typedef struct CAreaItem {
  const char *name;
  const char *code;
  unsigned char leaf;
} CAreaItem;

typedef struct CAreaItemVec {
  struct CAreaItem *data;
  uintptr_t len;
  uintptr_t capacity;
} CAreaItemVec;

typedef struct CAreaItemVecs {
  struct CAreaItemVec *data;
  uintptr_t len;
  uintptr_t capacity;
} CAreaItemVecs;

typedef struct CAreaRelatedItem {
  const char *name;
  const char *code;
  unsigned char selected;
  unsigned char leaf;
} CAreaRelatedItem;

typedef struct CAreaRelatedItemVec {
  struct CAreaRelatedItem *data;
  uintptr_t len;
  uintptr_t capacity;
} CAreaRelatedItemVec;

typedef struct CAreaRelatedItemVecs {
  struct CAreaRelatedItemVec *data;
  uintptr_t len;
  uintptr_t capacity;
} CAreaRelatedItemVecs;

/**
 * # Safety
 *
 * 用于外部C函数调用进行初始化结构
 * 不要在RUST调用
 *
 */
int area_db_init_csv(const char *code_path,
                     const char *geo_path,
                     const unsigned char *gz,
                     struct CAreaDao **area_dao,
                     char **error);

/**
 * # Safety
 *
 * 用于外部C函数调用进行初始化结构
 * 不要在RUST调用
 *
 */
int area_db_init_sqlite(const char *db_path, struct CAreaDao **area_dao, char **error);

/**
 * # Safety
 *
 * 用于外部C函数调用进行初始化结构
 * 不要在RUST调用
 *
 */
int area_db_init_mysql(const char *db_uri, struct CAreaDao **area_dao, char **error);

/**
 * # Safety
 *
 * 用于外部C函数调用进行初始化结构
 * 不要在RUST调用
 *
 */
int area_db_code_reload(struct CAreaDao **area_dao, char **error);

/**
 * # Safety
 *
 * 用于外部C函数调用进行初始化结构
 * 不要在RUST调用
 *
 */
int area_db_geo_reload(struct CAreaDao **area_dao, char **error);

/**
 * # Safety
 *
 * 释放 AreaDao 内存用
 * 不要在RUST调用
 *
 */
void area_db_release_area_dao(struct CAreaDao *ptr);

/**
 * # Safety
 *
 * 释放 错误消息用
 * 不要在RUST调用
 *
 */
void area_db_release_error_str(char *ptr);

/**
 * # Safety
 *
 * 释放 CAreaItemVec 内存
 * 不要在RUST调用
 *
 */
void area_db_release_item_vec(struct CAreaItemVec *ptr);

/**
 * # Safety
 *
 * 查询指定CODE的子节点
 * 不要在RUST调用
 *
 */
int area_db_code_childs(const char *code_str,
                        struct CAreaDao *area_dao,
                        struct CAreaItemVec **out_data,
                        char **error);

/**
 * # Safety
 *
 * 查询指定CODE的详细
 * 不要在RUST调用
 *
 */
int area_db_code_find(const char *code_str,
                      struct CAreaDao *area_dao,
                      struct CAreaItemVec **out_data,
                      char **error);

/**
 * # Safety
 *
 * 释放 CAreaItemVecs 内存
 * 不要在RUST调用
 *
 */
void area_db_release_item_vecs(struct CAreaItemVecs *ptr);

/**
 * # Safety
 *
 * 搜索指定关键字
 * 不要在RUST调用
 *
 */
int area_db_code_search(const char *code_str,
                        unsigned int limit,
                        struct CAreaDao *area_dao,
                        struct CAreaItemVecs **out_data,
                        char **error);

/**
 * # Safety
 *
 * 释放 CAreaRelatedItemVecs 内存
 * 不要在RUST调用
 *
 */
void area_db_release_related_vecs(struct CAreaRelatedItemVecs *ptr);

/**
 * # Safety
 *
 * 根据地区CODE查询地址数据
 * 不要在RUST调用
 *
 */
int area_db_code_related(const char *code_str,
                         struct CAreaDao *area_dao,
                         struct CAreaRelatedItemVecs **out_data,
                         char **error);

/**
 * # Safety
 *
 * 根据地区CODE查询地址数据
 * 不要在RUST调用
 *
 */
int area_db_geo_search(float lat,
                       float lng,
                       struct CAreaDao *area_dao,
                       struct CAreaItemVec **out_data,
                       char **error);
