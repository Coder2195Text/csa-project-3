use godot::engine::input::MouseMode;
use godot::engine::{Engine, ICharacterBody2D, InputEvent, InputEventMouseMotion, Node2D};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::settings::SettingsSingleton;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct GameUI {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for GameUI {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn process(&mut self, delta: f64) {
        let mut input = Input::singleton();
        let mut settings = Engine::singleton()
            .get_singleton("SettingsSingleton".into())
            .unwrap()
            .try_cast::<SettingsSingleton>()
            .unwrap();
        if input.is_action_just_pressed("paused".into()) {
            settings.call("toggle_pause".into(), &[]);
        }

        let paused = settings
            .call("get_paused".into(), &[])
            .try_to::<bool>()
            .unwrap();

        // write menu code

        if paused {
            input.set_mouse_mode(MouseMode::VISIBLE);
        } else {
            input.set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    fn physics_process(&mut self, delta: f64) {}
}
