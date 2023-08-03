
use capability_behaviour_rs::compartments::with_new_stack;
use capability_generate::{print_ast};

// usual perms :: field in abstract cap 
// revokable permission :: field in abstract cap
// ownership :: field in abstract cap
// range :: field in abstract cap
// refrence counting / global / local / reachability analysis 
//  -- ip limiting example 


#[print_ast]
struct Alpha<'a,'b> {
    value: &'a str,
    value2: &'b str,
    b: u8,
}

fn main() {
    let my_string = String::from("Hello, lifetimes!");


    println!("ok");
    with_new_stack();
    println!("ok doky {}", Alpha {  value: &my_string ,value2: &my_string,  b: 3 }.b);
}