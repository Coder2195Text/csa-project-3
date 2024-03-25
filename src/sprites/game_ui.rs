use godot::engine::input::MouseMode;
use godot::engine::{Control, Engine, IControl};
use godot::obj::WithBaseField;
use godot::prelude::*;

use crate::settings::SettingsSingleton;

#[derive(GodotClass)]
#[class(base=Control)]
pub struct GameUI {
    base: Base<Control>,
}

#[godot_api]
impl IControl for GameUI {
    fn init(base: Base<Control>) -> Self {
        Self { base }
    }

    fn process(&mut self, _delta: f64) {
        let mut input = Input::singleton();
        let mut settings = Engine::singleton()
            .get_singleton("SettingsSingleton".into())
            .unwrap()
            .try_cast::<SettingsSingleton>()
            .unwrap();
        if input.is_action_just_pressed("pause".into()) {
            settings.call("toggle_pause".into(), &[]);
        }

        let paused = settings
            .call("get_paused".into(), &[])
            .try_to::<bool>()
            .unwrap();

        self.base().get_tree().unwrap().set_pause(paused);

        let mut menu = self.base().get_node_as::<Node2D>("Menu");

        if paused {
            menu.show();
            input.set_mouse_mode(MouseMode::VISIBLE);
        } else {
            menu.hide();
            input.set_mouse_mode(MouseMode::CAPTURED);
        }
    }

    fn physics_process(&mut self, _delta: f64) {}
}
