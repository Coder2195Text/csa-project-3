use godot::{engine::ResourceLoader, prelude::*};

#[derive(GodotClass)]
#[class(init,base=Node3D)]
pub struct BallSpawner {
    base: Base<Node3D>,
    #[export]
    pub balls: i32,
}

#[godot_api]
impl INode3D for BallSpawner {
    fn ready(&mut self) {
        let balls = self.balls;
        let mut base = self.base_mut();
        let ball = ResourceLoader::singleton()
            .load("res://sprites/Ball.tscn".into())
            .unwrap()
            .try_cast::<PackedScene>()
            .unwrap();

        for _ in 0..balls {
            let ball = ball.instantiate().unwrap();
            base.add_child(ball);
        }
    }
}
