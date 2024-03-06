use godot::obj::WithBaseField;
use godot::prelude::*;
use godot::engine::{CharacterBody3D, ICharacterBody2D, InputEvent, InputEventMouseMotion, Viewport};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
        }
    }

    fn input_event(&mut self, viewport: Gd<Viewport>, input: Gd<InputEvent>, _shape_idx: i32) {
        if (input.is_class("InputEventMouseMotion".into())) {
            self.base_mut().rotate_y(input.try_cast::<InputEventMouseMotion>().unwrap().get_relative().x * -0.01);
        }
    }
}
