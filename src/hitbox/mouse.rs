use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Resource, Default)]
pub struct MouseCoords(pub Vec2);

impl From<&MouseCoords> for Vec2 {
    fn from(mouse_coords: &MouseCoords) -> Vec2 {
        mouse_coords.0
    }
}

pub fn mouse_coords_system(
    mut mouse_coords: ResMut<MouseCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok((camera, camera_transform)) = q_camera.get_single() else { return };

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.get_single().expect("No window found");

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        mouse_coords.0 = world_position;
    }
}