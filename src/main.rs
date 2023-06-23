
use morello::capability::*;
use cheribsd_capability_rs::cheribsd::get_root_seal;

fn print_cap(cap: *const i64, name: &str) {

    // Extract the address portion of &arr
    let addr1_usize : usize = get_address(cap);
    let perms = get_perms(cap);
    println!("&arr  address: {},   bounds: {}, for {}, {} ", addr1_usize, get_length(&cap), name, perms);

    // unsafe {
    //     // Dereference our restricted capability
    //     println!("{}", *cap);
    // }
}

fn main() {
    let mut value : i64 = 0;

 //   unsafe  {
    let  mut root_seal = &mut value as *mut i64;

    print_cap(root_seal, "asdf");

    let mut size = std::mem::size_of_val(&root_seal);
    let result = get_root_seal(&mut root_seal, &mut size as *mut usize);

    match result {
        Ok(_value) => {
            println!("ok");


            print_cap(root_seal, "done");
       }
        Err(error) => {
            println!("Error: {}", error);
        }
    }


   // }



}