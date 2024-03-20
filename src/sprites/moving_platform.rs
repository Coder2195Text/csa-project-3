use godot::engine::{AnimatableBody3D, IAnimatableBody3D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=AnimatableBody3D)]
pub struct MovingPlatform {
    base: Base<AnimatableBody3D>,
    #[export]
     speed: f32,
    #[export]
    distance: f32,
    home: Vector3,

}

#[godot_api]
impl  IAnimatableBody3D for MovingPlatform {
    fn init(base: Base<AnimatableBody3D>) -> Self {

        Self {
            base,
            speed: 1.0,
            distance: 1.0,
            home: Vector3::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let base = self.base();
        let pos = base.get_position();
        let new_pos = pos + Vector3::new(self.speed * delta as f32, 0.0, 0.0);
        
        
        if (new_pos - self.home).length() > self.distance && self.speed > 0.0 || (new_pos - self.home).length() < 0.0 && self.speed < 0.0 {
            self.speed *= -1.0;
        }

        godot_print!("new_pos: {:?}", new_pos);
        godot_print!("home: {:?}", self.home);
        godot_print!("distance: {:?}", (new_pos - self.home).length());

        let mut base = self.base_mut();
        base.set_position(new_pos);
    }
}

#[godot_api]
impl MovingPlatform {
    #[func]
    fn ready(&mut self) {
        godot_print!("ready");
        self.home = self.base().get_position();
    }
    
}
