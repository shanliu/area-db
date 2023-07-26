PHP_ARG_WITH([area_db],
    [for area_db support],
    [AS_HELP_STRING([--with-area_db],[Include area_db support])])

PHP_ARG_WITH(area_db_dir,
    for area_db,
    [AS_HELP_STRING([--with-area_db_dir=PATH],[set area db lib dir])],
    yes)

PHP_ARG_WITH(area_db_use_sqlite,
    for area_db,
    [AS_HELP_STRING([--with-area_db_use_sqlite],[enable sqlite data source support, You need to enable data-sqlite in cargo.toml first.])],
    no)
PHP_ARG_WITH(area_db_use_mysql,
    for area_db,
    [AS_HELP_STRING([--with-area_db_use_mysql],[enable mysql data source support, You need to enable data-mysql in cargo.toml first.])],
    no)

if test "$PHP_AREA_DB" != "no"; then

   if test "$PHP_AREA_DB_DIR" = "yes"; then
   		if test -r ./lib/area_db.h;
   		then
   			PHP_ADD_INCLUDE(./lib)
   		else
   			AC_MSG_ERROR([area header file not found])
   		fi
        if test -r ./lib/libarea_db.so;
        then
            PHP_ADD_LIBRARY_WITH_PATH(area_db, ./lib/, AREA_DB_SHARED_LIBADD)
        else
            AC_MSG_ERROR([zxing path ./lib/ not find libarea_db.so])
        fi
   	else
   		if test -r $PHP_AREA_DB_DIR/area_db.h;
   		then
   			PHP_ADD_INCLUDE($PHP_AREA_DB_DIR)
   		else
   			AC_MSG_ERROR([area db path not find area_db.h])
   		fi
   		if test -r $PHP_AREA_DB_DIR/libarea_db.so;
        then
            PHP_ADD_LIBRARY_WITH_PATH(area_db, $PHP_AREA_DB_DIR, AREA_DB_SHARED_LIBADD)
        else
            AC_MSG_ERROR([zxing path $PHP_AREA_DB_DIR not find libarea_db.so])
        fi
   	fi
   PHP_SUBST(AREA_DB_SHARED_LIBADD)

   if test "$PHP_AREA_DB_USE_SQLITE" != "no"; then
        AC_DEFINE(HAVE_AREA_DB_USE_SQLITE, 1, [ Have area_db SQLITE support ])
   fi

    if test "$PHP_AREA_DB_USE_MYSQL" != "no"; then
        AC_DEFINE(HAVE_AREA_DB_USE_MYSQL, 1, [ Have area_db MYSQL support ])
    fi

   AC_DEFINE(HAVE_AREA_DB, 1, [ Have area_db support ])
   PHP_NEW_EXTENSION(area_db, area_db.c area_db_class.c area_db_exception_class.c, $ext_shared)
fi
