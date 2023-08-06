
use capability_behaviour_rs::compartments::with_new_stack;
use capability_generate::{print_ast,HelloMacro};
use std::collections::HashSet;
use std::sync::{Mutex, Arc};
use lazy_static::{lazy_static};

// usual perms :: field in abstract cap 
// revokable permission :: field in abstract cap
// ownership :: field in abstract cap
// range :: field in abstract cap
// refrence counting / global / local / reachability analysis 
//  -- ip limiting example 


mod my_opaque {
    use super::*;

    #[derive(Debug)]
    pub struct Opaque(i32);

    
    impl Opaque {
        pub fn new(value: i32) -> Opaque {
            // Perform any validation or preprocessing here.

            register_value(value);

            Opaque(value)
        }
    }

}

lazy_static! {
    // The global thread-safe HashSet to store the constructed values.
    static ref CONSTRUCTED_VALUES: Mutex<HashSet<i32>> = Mutex::new(HashSet::new());
}

fn register_value(value: i32) {
    let mut set = CONSTRUCTED_VALUES.lock().unwrap();
    set.insert(value);
}


pub trait HelloMacro {
    fn hello_macro();
}

#[print_ast]
#[derive(HelloMacro)]
struct Alpha<'a> {
    value: &'a str,
    b: u8,
}


fn main() {
    let my_string = String::from("Hello, lifetimes!");


    println!("ok");
    with_new_stack();
    println!("ok doky {}", Alpha { value: &my_string , b: 3 }.value);


    let instance = my_opaque::Opaque::new(42);
    println!("Constructed opaque instance: {:?}", instance);

    let instance2 = my_opaque::Opaque::new(42);
    println!("Constructed opaque instance: {:?}", instance2);

    // should fail 
    // println!("Constructed opaque instance: {:?}", my_opaque::Opaque(33));

        // Print the stored values
        let set = CONSTRUCTED_VALUES.lock().unwrap();
        println!("Stored values: {:?}", *set);

}