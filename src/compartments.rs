use std::alloc::{alloc, dealloc, Layout};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
// use libc::{c_int, c_void, c_char};
use morello::capability::*;
use cheribsd_capability_rs::cheribsd::get_root_seal;

// https://github.com/vanjacosic/rust-ffi-to-c
// https://doc.rust-lang.org/nomicon/ffi.html
// https://github.com/kent-weak-memory/rust/commit/16fafcd791266ac9eb31c3122007694582c94dcd
extern crate libc;

extern "C" {
    fn double_input(input: *const usize) -> libc::c_int;
}


fn print_cap(cap: *const i64, name: &str) {

    let addr1_usize : usize = get_address(cap);
    let perms = get_perms(cap);

    println!("&arr  address: {},   bounds: {}, perms {}, for {} ", addr1_usize, get_length(&cap), perms, name);
}

fn int_to_program_pointer(pointer: usize) -> *const c_void {
    let capability: *const c_void;
    unsafe {
        asm!("cvtpz {0}, {1}", out(reg) capability, in(reg) pointer);
    }
    capability
}

pub fn with_new_stack() {

    // print_cap(unsafe { foobar as *const i64 }, "foobar");
    
        // print_cap(double_input as *const i64, "asdf 2");
        let input = 16;
        let cap = unsafe { int_to_program_pointer(input) };

        let output = unsafe { double_input(&input as *const usize) };
        println!("{} * 2 = {}", input, output);
        // print_cap(double_input as *const i64, "asdf");
    
    println!("string: {}", 3);
}