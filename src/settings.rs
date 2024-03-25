use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct SettingsSingleton {
    #[var]
    pub paused: bool,
    #[var]
    pub sensitivity: f32,
    #[var]
    pub volume: f32,
    base: Base<Object>,
}

#[godot_api]
impl IObject for SettingsSingleton {
    fn init(base: Base<Object>) -> Self {
        Self {
            paused: false,
            sensitivity: 1.0,
            volume: 1.0,
            base,
        }
    }
}

#[godot_api]
pub impl SettingsSingleton {
    #[func]
    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}
