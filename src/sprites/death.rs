use godot::engine::Area3D;
use godot::obj::WithBaseField;
use godot::prelude::*;

use super::player::Player;

#[derive(GodotClass)]
#[class(init,base=Area3D)]
pub struct Death {
    base: Base<Area3D>,
}

#[godot_api]
impl Death {
    #[func]
    fn on_touched(&mut self, body: Gd<Node3D>) {
        if let Ok(mut player) = body.try_cast::<Player>() {
            player.call("die".into(), &[]);
        }
    }
}
