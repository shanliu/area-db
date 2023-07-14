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
    int len1=area_vec1->len;
    CAreaItem * tmp1=area_vec1->data;
    while (len1-->0){
        printf("%s [%s]\n",tmp1->name,tmp1->code);
        tmp1++;
    }
    release_area_vec(area_vec1);

    CAreaItemVec* area_vec;
    if (code_detail("441403131",area_dao,&area_vec,&err)!=0){
        printf("err:%s",err);
        release_error(err);
        return 0;
    }
    int len=area_vec->len;
    CAreaItem * tmp=area_vec->data;
    while (len-->0){
        printf("%s [%s]\n",tmp->name,tmp->code);
        tmp++;
    }
   release_area_vec(area_vec);
   release_area(area_dao);
   return 0;
}
