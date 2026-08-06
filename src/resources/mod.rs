use bevy::input::keyboard::Key;
use bevy::prelude::{KeyCode, Resource, Vec3};

#[derive(Resource, Default)]
pub struct CameraController {
    target: Vec3,
    distance: f32,
    yaw: f32,
    pitch: f32,
}
#[derive(Resource)]
pub struct Keybinds{
    pub camera_forwards: Vec<KeyCode>,
    pub camera_backwards: Vec<KeyCode>,
    pub camera_right: Vec<KeyCode>,
    pub camera_left: Vec<KeyCode>,
    pub camera_zoom_in: Vec<KeyCode>,
    pub camera_zoom_out: Vec<KeyCode>,
}
impl Default for Keybinds{
    fn default() -> Self {
        Self{
            camera_forwards: vec![KeyCode::KeyW, KeyCode::ArrowUp],
            camera_backwards: vec![KeyCode::KeyS, KeyCode::ArrowDown],
            camera_right: vec![KeyCode::KeyD, KeyCode::ArrowRight],
            camera_left: vec![KeyCode::KeyA, KeyCode::ArrowLeft],
            camera_zoom_in: vec![KeyCode::KeyX, KeyCode::Equal],
            camera_zoom_out: vec![KeyCode::KeyZ, KeyCode::Minus],
        }
    }
}