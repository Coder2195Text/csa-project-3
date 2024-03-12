use godot::engine::input::MouseMode;
use godot::engine::{
    Area3D, IArea3D, InputEvent, InputEventMouseMotion,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

use super::player::Player;

#[derive(GodotClass)]
#[class(init,base=Area3D)]
pub struct Spawn {
    base: Base<Area3D>,
    #[export]
    pub level: i32
}


#[godot_api]
impl Spawn {
    #[func]
    fn on_touched(&self, body: Gd<Node3D>) {
        godot_print!("Spawned");
        if let Ok(mut player) = body.try_cast::<Player>() {
           player.call("set_spawn".into(), &[Variant::from(self.to_gd())]);
        }
    }

    #[func]
    fn success(&self) {
        godot_print!("Success");
    }
}
