<?php
final class LibOneBrc {
    private static $ffi = null;

    function __construct() {
        if (is_null(self::$ffi)) {
            self::$ffi = FFI::cdef("char* run(const char* str);", "rust/libonebrc.so");
        }
    }

    function run($filename) {
       $resultPtr = self::$ffi->run($filename);
       return FFI::string($resultPtr);
    }
}
