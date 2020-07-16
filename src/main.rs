use cake::*;

fn main() {
    let mut scene = Scene::new();
    let mut entity = Entity::new();
    entity
        .attach(TestBehavior())
        .attach(TestBehavior())
        .attach(TestBehavior())
        .attach(TestBehavior());
    scene.add_entity(entity);
    start();
}

struct TestSystem();

impl System for TestSystem {
    type Data = ();

    fn update(&mut self, data: Self::Data) {}
}

#[derive(Component)]
struct TestBehavior();

impl Behavior for TestBehavior {
    fn update(&self) {}
}
