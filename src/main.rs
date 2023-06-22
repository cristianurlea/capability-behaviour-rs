use std::ptr;
use morello::capability::*;
use cheribsd_capability_rs::cheribsd::get_root_seal;

fn print_cap(cap: Capability, name: &str) {

    // Extract the address portion of &arr
    let addr1_usize : usize = get_address(&cap);

    println!("&arr  address: {},   bounds: {}, for {}", addr1_usize, get_length(&cap), name);

    // unsafe {
    //     // Dereference our restricted capability
    //     println!("{}", *cap);
    // }
}

fn main() {
    
    let root_seal: *mut Capability = ptr::null_mut() as *mut Capability;
    let size = std::mem::size_of_val(&root_seal);

    let result = get_root_seal(root_seal, size);

    match result {
        Ok(_value) => {

            let cap_val = unsafe { *root_seal };
            print_cap(cap_val , "example");

        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }}

