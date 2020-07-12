use cake::*;

fn main() {
    let e = Entity::new();
    start();
}

pub fn awesome_function(number:f64) -> f64 {
    println!("asdsadsad {}", number);
    number
}

struct TestSystem ();

impl System for TestSystem {
    
}

#[derive(Component)]
struct TestBehavior ();

impl Behavior for TestBehavior {

}
