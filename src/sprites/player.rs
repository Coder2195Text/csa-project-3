use godot::engine::input::MouseMode;
use godot::engine::{CharacterBody3D, ICharacterBody2D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    #[export]
    pub last_spawn: Vector3,
    #[export]
    pub last_rotation: Vector3,
    pub last_level: i32,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            last_spawn: Vector3::new(0.0, 0.0, 0.0),
            last_rotation: Vector3::new(0.0, 0.0, 0.0),
            last_level: 0,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let mut base = self.base_mut();
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
            base.rotate_y(event.get_relative().x * -0.005);
            let mut camera = base.get_node_as::<Camera3D>("Camera");
            let rot = camera.get_rotation_degrees().x;
            let move_rot = event.get_relative().y * -0.01;
            if move_rot == 0.0 {
                return;
            }
            if (rot + move_rot > -90.0 && event.get_relative().y > 0.0)
                || (rot + move_rot < 90.0 && event.get_relative().y < 0.0)
            {
                camera.rotate_x(move_rot);
            } else {
                camera.set_rotation_degrees(Vector3::new(
                    if rot + move_rot > 0.0 { 90.0 } else { -90.0 },
                    0.0,
                    0.0,
                ));
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let pos = self.base().get_global_position();
        if pos.y < -60.0 {
            self.die();
        }

        let mut base = self.base_mut();
        let mut velocity = Vector3::new(0.0, base.get_velocity().y, 0.0);
        let input = Input::singleton();

        if input.is_action_pressed("forward".into()) {
            velocity += -base.get_transform().basis.col_c() * 7.5;
        }
        if input.is_action_pressed("back".into()) {
            velocity += base.get_transform().basis.col_c() * 5.0;
        }
        if input.is_action_pressed("left".into()) {
            velocity += -base.get_transform().basis.col_a() * 6.0;
        }
        if input.is_action_pressed("right".into()) {
            velocity += base.get_transform().basis.col_a() * 5.0;
        }
        if input.is_action_pressed("jump".into()) && base.is_on_floor() {
            velocity.y = 12.0;
        }

        velocity.y -= 30.0 * delta as f32;
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}

#[godot_api]
impl Player {
    #[func]
    pub fn set_spawn(&mut self, spawn: Vector3, rotation: Vector3, level: i32) -> bool {
        if level <= self.last_level {
            return false;
        }
        self.last_level = level;

        self.last_spawn = spawn;
        self.last_rotation = rotation;
        true
    }

    #[func]
    fn die(&mut self) {
        let mut cam = self.base().get_node_as::<Camera3D>("Camera");
        let mut death_sound = self.base().get_node_as::<AudioStreamPlayer>("DeathSound");
        death_sound.play();
        let pos = self.last_spawn;
        let rot = self.last_rotation;

        let mut base = self.base_mut();
        base.set_global_position(pos);
        base.set_velocity(Vector3::ZERO);
        base.set_rotation(Vector3::new(0.0, rot.y, 0.0));
        cam.set_rotation(Vector3::new(rot.x, 0.0, 0.0));
    }
}
