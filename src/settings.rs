use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct SettingsSingleton {
    #[var]
    paused: bool,
    #[var]
    sensitivity: f32,
    #[var]
    volume: f32,
    base: Base<Object>,
}

#[godot_api]
pub impl SettingsSingleton {
    
}