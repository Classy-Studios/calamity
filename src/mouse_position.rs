use {
    super::primary_camera::PrimaryCamera,
    bevy::{prelude::*, window::PrimaryWindow},
};

#[derive(Resource)]
pub struct MousePosition(Vec2);

impl MousePosition {
    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn as_vec(&self) -> Vec2 {
        self.0
    }
}

fn update_mouse_position(
    mut old_mouse_pos: ResMut<MousePosition>,
    primary_win_qry: Query<&Window, With<PrimaryWindow>>,
    primary_cam_qry: Query<(&Camera, &GlobalTransform), With<PrimaryCamera>>,
) {
    let primary_win = primary_win_qry.single();
    let (primary_cam, primary_cam_glob_xform) = primary_cam_qry.single();

    let Some(new_mouse_pos) = primary_win
        .cursor_position()
        .and_then(|mouse_pos| primary_cam.viewport_to_world_2d(primary_cam_glob_xform, mouse_pos))
    else {
        return;
    };
    old_mouse_pos.0 = new_mouse_pos;
}

pub fn mouse_position_plugin(app: &mut App) {
    app.insert_resource(MousePosition(Vec2::ZERO))
        .add_systems(Update, update_mouse_position);
}
