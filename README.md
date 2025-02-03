## Enable FFI in PHP 8.3 (Arch Linux)
```sh
sudo sed -i 's/;extension=ffi/extension=ffi/' /etc/php/php.ini
sudo sed -i 's/;zend_extension=opcache/zend_extension=opcache/' /etc/php/php.ini

echo "ffi.enable=preload" | sudo tee -a /etc/php/conf.d/ffi.ini

echo "opcache.enable=1" | sudo tee -a /etc/php/conf.d/opcache.ini
echo "opcache.enable_cli=1" | sudo tee -a /etc/php/conf.d/opcache.ini
echo "opcache.jit=on" | sudo tee -a /etc/php/conf.d/opcache.ini
```

## Compiling Rust library
From the root directory, run:  
```sh
cd rust/onebrc && cargo build --release && rm -f ../libonebrc.so && mv target/release/libonebrc.so ../ && strip --strip-unneeded ../libonebrc.so && cd ../../
```

## Running
Generate `rust/measurements.txt` file from the original 1brc tool.  
Then from the root directory, run:  

```sh
php app/index.php
```
