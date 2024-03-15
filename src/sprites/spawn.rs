use godot::engine::{Area3D, GpuParticles3D};
use godot::obj::WithBaseField;
use godot::prelude::*;

use super::player::Player;

#[derive(GodotClass)]
#[class(init,base=Area3D)]
pub struct Spawn {
    base: Base<Area3D>,
    #[export]
    pub level: i32,
}

#[godot_api]
impl Spawn {
    #[func]
    fn on_touched(&mut self, body: Gd<Node3D>) {
        godot_print!("Spawned");
        let base = self.base();
        if let Ok(mut player) = body.try_cast::<Player>() {
            let result = player
                .call("set_spawn".into(), &[Variant::from(base.get_global_position()), Variant::from(base.get_rotation()), Variant::from(self.level)])
                .try_to::<bool>()
                .unwrap();
            if result {
                self.success();
            }
        }
    }

    #[func]
    fn success(&mut self) {
        godot_print!("Success");
        let base = self.base_mut();
        base.get_node_as::<GpuParticles3D>("Confetti")
            .set_emitting(true);
    }
}
