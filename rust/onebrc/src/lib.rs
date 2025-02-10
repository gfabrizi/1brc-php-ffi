/**
 * Courtesy of Flavio Bizzarri ( https://github.com/newfla/1brc_rust ) with minor fixes for using as FFI
 */
use std::fmt::Display;

extern crate libc;
use libc::c_char;
use std::ffi::CStr;

use serde::Serialize;

pub mod adv;

#[derive(Debug, Clone, Serialize)]
struct WeatherRecord {
    city: String,
    min: i32,
    max: i32,
    sum: i64,
    count: u32,
}

impl WeatherRecord {
    fn update(&mut self, item: i32) {
        self.count += 1;
        self.min = self.min.min(item);
        self.max = self.max.max(item);
        self.sum += item as i64;
    }

    fn new(city: &str, item: i32) -> Self {
        Self {
            city: city.to_owned(),
            min: item,
            max: item,
            sum: item as i64,
            count: 1,
        }
    }

    fn merge(&mut self, rhs: &Self) {
        self.count += rhs.count;
        self.min = self.min.min(rhs.min);
        self.max = self.max.max(rhs.max);
        self.sum += rhs.sum;
    }
}

impl Display for WeatherRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mean = self.sum as f32 / self.count as f32;
        write!(
            f,
            "{}: {:.1}/{:.1}/{:.1}",
            self.city,
            self.min as f32 / 10.,
            mean / 10.,
            self.max as f32 / 10.
        )
    }
}

#[no_mangle]
pub extern "C" fn run(filename: *const c_char) -> *mut c_char {
    /*
     * Converting a c-style string into a Rust string
     *
     * https://stackoverflow.com/questions/24145823/how-do-i-convert-a-c-string-into-a-rust-string-and-back-via-ffi
     * http://jakegoulding.com/rust-ffi-omnibus/string_arguments/
     * https://crates.io/crates/libc
     */
    let c_str = unsafe {
        assert!(!filename.is_null());
        CStr::from_ptr(filename)
    };
    let path = c_str.to_str().unwrap().to_string();

    adv::process(path)
}
