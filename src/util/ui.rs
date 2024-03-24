use bevy::math::Vec2;
use bevy::prelude::{OrthographicProjection, Transform, Window};
use bevy::render::camera::CameraProjection;

pub(crate) fn ui_to_world(
    pos: Vec2,
    window: &Window,
    camera_transform: &Transform,
    camera_projection: &OrthographicProjection,
) -> Vec2 {
    // Get the size of the window
    let window_size = Vec2::new(window.width(), window.height());

    // Convert cursor position to NDC (Normalized Device Coordinates)
    let ndc = (pos / window_size) * 2.0 - Vec2::ONE;

    // Convert NDC to world coordinates
    let world_pos = camera_transform.compute_matrix()
        * camera_projection.get_projection_matrix().inverse()
        * ndc.extend(-1.0).extend(1.0);

    // invert y to match world coordinates
    Vec2::new(world_pos.x, world_pos.y)
}
