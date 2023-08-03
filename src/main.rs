
use capability_behaviour_rs::compartments::with_new_stack;
use capability_generate::{print_ast};

// usual perms :: field in abstract cap 
// revokable permission :: field in abstract cap
// ownership :: field in abstract cap
// range :: field in abstract cap
// refrence counting / global / local / reachability analysis 
//  -- ip limiting example 

#[print_ast]
struct Alpha {
    b: u8,
}

fn main() {
    println!("ok");
    with_new_stack();
    println!("ok doky {}", Alpha { b: 3 }.b);
}