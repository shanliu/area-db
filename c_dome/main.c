#include <stdio.h>
#include "area_lib.h"
int main() {
    CAreaDao* area_dao;
    char * err;
    int ret=init_area_csv("","",&area_dao,&err);
    if (ret!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }

    CAreaItemVec* area_vec1;
    if (code_childs("",area_dao,&area_vec1,&err)!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }
    u_long len1=area_vec1->len;
    CAreaItem * tmp1=area_vec1->data;
    while (len1-->0){
        printf("%s [%s]\n",tmp1->name,tmp1->code);
        tmp1++;
    }
    release_area_item_vec(area_vec1);

    int ret2=reload_area_dao(&area_dao,&err);
    if (ret2!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }

    CAreaItemVec* area_vec2;
    if (code_childs("",area_dao,&area_vec2,&err)!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }
    u_long len2=area_vec2->len;
    CAreaItem * tmp2=area_vec2->data;
    while (len2-->0){
        printf("%s [%s]\n",tmp2->name,tmp2->code);
        tmp2++;
    }
    release_area_item_vec(area_vec2);


    CAreaItemVec* area_vec;
    if (code_find("441403131",area_dao,&area_vec,&err)!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }
    u_long len=area_vec->len;
    CAreaItem * tmp=area_vec->data;
    while (len-->0){
        printf("%s [%s]\n",tmp->name,tmp->code);
        tmp++;
    }
    release_area_item_vec(area_vec);
    release_area_dao(area_dao);
   return 0;
}
