use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
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
pub impl SettingsSingleton {
    #[func]
    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}
