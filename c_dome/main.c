#include <stdio.h>
#include "area_lib.h"
void test_code_related(CAreaDao* area_dao){
    char * err;
    printf("code related \n");
    CAreaRelatedItemVecs *area_vecr;
    if (area_db_code_related("4414",area_dao,&area_vecr,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    u_long lenr1 = area_vecr->len;
    CAreaRelatedItemVec *tmpr1 = area_vecr->data;
    while (lenr1-- > 0) {
        u_long len2 = tmpr1->len;
        CAreaRelatedItem *tmp2 = tmpr1->data;
        while (len2-- > 0) {
            printf("%s:%s\n",tmp2->name,tmp2->selected?"[*]":"[ ]");
            tmp2++;
        }
        tmpr1++;
    }
    area_db_release_related_vecs(area_vecr);

    CAreaRelatedItemVecs *tmpt;
    if (area_db_code_related("121212121212",area_dao,&tmpt,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    area_db_release_related_vecs(tmpt);

}
void test_reload(CAreaDao* area_dao){
    char * err;

    printf("code reload \n");
    int ret2=area_db_code_reload(&area_dao,&err);
    if (ret2!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }

    printf("geo reload \n");
    int ret3=area_db_geo_reload(&area_dao,&err);
    if (ret3!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
}

void test_code_find(CAreaDao* area_dao){
    char * err;
    printf("code find \n");
    CAreaItemVec* area_vec;
    if (area_db_code_find("441403131",area_dao,&area_vec,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    u_long len=area_vec->len;
    CAreaItem * tmp=area_vec->data;
    while (len-->0){
        printf("%s [%s]\n",tmp->name,tmp->code);
        tmp++;
    }
    area_db_release_item_vec(area_vec);




    CAreaItemVec* tmp1;
    if (area_db_code_find("121212121212",area_dao,&tmp1,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    area_db_release_item_vec(tmp1);
}


void test_code_childs(CAreaDao* area_dao){
    char * err;
    printf("code childs \n");
    CAreaItemVec* area_vec2;
    if (area_db_code_childs("",area_dao,&area_vec2,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    u_long len2=area_vec2->len;
    CAreaItem * tmp2=area_vec2->data;
    while (len2-->0){
        printf("%s [%s]\n",tmp2->name,tmp2->code);
        tmp2++;
    }
    area_db_release_item_vec(area_vec2);


    CAreaItemVec* tmp;
    if (area_db_code_childs("12213123123123",area_dao,&tmp,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    area_db_release_item_vec(tmp);
}

void test_geo_search(CAreaDao* area_dao){
    char * err;
    printf("geo search \n");
    CAreaItemVec* area_vecg;
    if (area_db_geo_search(22.57729, 113.89409,area_dao,&area_vecg,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    u_long leng=area_vecg->len;
    CAreaItem * tmpg=area_vecg->data;
    while (leng-->0){
        printf("%s [%s]\n",tmpg->name,tmpg->code);
        tmpg++;
    }
    area_db_release_item_vec(area_vecg);

    CAreaItemVec* tmp;
    if (area_db_geo_search(0.0, 0.0,area_dao,&tmp,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    area_db_release_item_vec(tmp);
}

void test_code_search(CAreaDao* area_dao){
    char * err;
    printf("code search \n");
    CAreaItemVecs* area_vec1;
    if (area_db_code_search("guangdong",10,area_dao,&area_vec1,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    u_long len1 = area_vec1->len;
    CAreaItemVec *tmp1 = area_vec1->data;
    while (len1-- > 0) {
        u_long len2 = tmp1->len;
        CAreaItem *tmp2 = tmp1->data;
        printf("address:");
        while (len2-- > 0) {
            printf("%s ",tmp2->name);
            tmp2++;
        }
        printf("\n");
        tmp1++;
    }
    area_db_release_item_vecs(area_vec1);



    CAreaItemVecs* tmp;
    if (area_db_code_search("fadsfasdadfasd",10,area_dao,&tmp,&err)!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return ;
    }
    area_db_release_item_vecs(tmp);


}


int main() {
    CAreaDao* area_dao;
    char * err;
    int ret=area_db_init_csv("","",&area_dao,&err);
    if (ret!=0){
        printf("err:%s\n",err);
        area_db_release_error_str(err);
        return 0;
    }
    test_code_search(area_dao);
    test_code_childs(area_dao);
    test_code_find(area_dao);
    test_code_related(area_dao);
    test_geo_search(area_dao);
    test_reload(area_dao);
    test_code_search(area_dao);
    test_code_childs(area_dao);
    test_code_find(area_dao);
    test_code_related(area_dao);
    test_geo_search(area_dao);
    area_db_release_area_dao(area_dao);
    return 0;
}
