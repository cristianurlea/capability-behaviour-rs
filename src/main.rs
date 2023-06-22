use std::ptr;
use morello::capability::*;
use cheribsd_capability_rs::cheribsd::get_root_seal;

fn print_cap(cap: *const i64, name: &str) {

    // Extract the address portion of &arr
    let addr1_usize : usize = get_address(&cap);

    println!("&arr  address: {},   bounds: {}, for {}", addr1_usize, get_length(&cap), name);

    // unsafe {
    //     // Dereference our restricted capability
    //     println!("{}", *cap);
    // }
}

fn main() {
    let mut value : i64 = 0;

    let root_seal: *const i64 = &value;
    let size = std::mem::size_of_val(&root_seal);
    let result = get_root_seal(root_seal, size);

    match result {
        Ok(_value) => {

            // let cap_val = unsafe { *root_seal };
            print_cap(root_seal , "example");

        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }}

