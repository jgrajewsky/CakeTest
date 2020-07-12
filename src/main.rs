use cake::*;

fn main() {
    cake::start();
}

struct TestSystem ();

impl System for TestSystem {
    
}

#[derive(Component)]
struct TestBehavior ();

impl Behavior for TestBehavior {

}
