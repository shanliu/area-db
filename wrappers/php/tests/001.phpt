--TEST--
Check if area_db is loaded
--SKIPIF--
<?php
if (!extension_loaded('area_db')) {
    echo 'skip';
}
?>
--FILE--
<?php
echo 'The extension "area_db" is available';
?>
--EXPECT--
The extension "area_db" is available
