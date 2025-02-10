<?php
final class LibOneBrc {
    private static $ffi = null;

    public function __construct()
    {
        if (is_null(self::$ffi)) {
            self::$ffi = FFI::cdef("char* run(const char* str);", "rust/libonebrc.so");
        }
    }

    public function run(string $filename): string
    {
       $resultPtr = self::$ffi->run($filename);
       return FFI::string($resultPtr);
    }
}
