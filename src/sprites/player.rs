use godot::engine::input::MouseMode;
use godot::engine::{
    CharacterBody3D, ICharacterBody2D, InputEvent, InputEventMouseMotion, Viewport,
};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self { base }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let mut base = self.base_mut();
        Input::singleton().set_mouse_mode(MouseMode::CAPTURED);
        if let Ok(event) = event.try_cast::<InputEventMouseMotion>() {
            base.rotate_y(event.get_relative().x * -0.005);
            let mut camera = base.get_node_as::<Camera3D>("Camera");
            let rot = camera.get_rotation_degrees().x;
            if (rot > -90.0 && event.get_relative().y > 0.0)
                || (rot < 90.0 && event.get_relative().y < 0.0)
            {
                camera.rotate_x(event.get_relative().y * -0.01);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut base = self.base_mut();
        let mut velocity = Vector3::new(0.0, base.get_velocity().y, 0.0);
        let input = Input::singleton();
       
        if input.is_action_pressed("forward".into()) {
            velocity += -base.get_transform().basis.col_c() * 6.0;
        }
        if input.is_action_pressed("back".into()) {
            velocity += base.get_transform().basis.col_c() * 6.0;
        }
        if input.is_action_pressed("left".into()) {
            velocity += -base.get_transform().basis.col_a() * 6.0;
        }
        if input.is_action_pressed("right".into()) {
            velocity += base.get_transform().basis.col_a() * 6.0;
        }
        if input.is_action_pressed("jump".into()) && base.is_on_floor() {
            velocity.y = 10.0;
        }

        
        velocity.y -= 15.0 * delta as f32;
        base.set_velocity(velocity);
        base.move_and_slide();
    }
}
