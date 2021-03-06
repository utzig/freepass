extern crate libc;

extern crate rustc_serialize;
extern crate cbor;
extern crate byteorder;
#[cfg(unix)] extern crate unix_socket;

extern crate secstr;
extern crate rusterpassword;
extern crate sodiumoxide;
extern crate rand;

extern crate chrono;


pub mod data;
pub mod output;

pub fn init() {
    sodiumoxide::init();
}
