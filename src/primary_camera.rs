use {
    super::{game_state::GameState, level::LEVEL_SIZE, player::Player, tile::TILE_SIZE},
    bevy::prelude::*,
};

#[derive(Component)]
pub struct PrimaryCamera;

fn follow_player(
    mut primary_cam_qry: Query<&mut Transform, With<PrimaryCamera>>,
    player_qry: Query<&Transform, (With<Player>, Without<PrimaryCamera>)>,
) {
    let mut primary_cam_xform = primary_cam_qry.single_mut();
    let player_xform = player_qry.single();

    primary_cam_xform.translation = player_xform
        .translation
        .truncate()
        .extend(primary_cam_xform.translation.z);
}

fn clamp_to_tilemap(
    mut primary_cam_qry: Query<
        (&Camera, &OrthographicProjection, &mut Transform),
        With<PrimaryCamera>,
    >,
) {
    let (primary_cam, primary_cam_proj, mut primary_cam_xform) = primary_cam_qry.single_mut();
    let Some(scaled_vp_size) = primary_cam
        .logical_viewport_size()
        .map(|logical_vp_size| logical_vp_size * primary_cam_proj.scale)
    else {
        return;
    };
    let tilemap_size_px = LEVEL_SIZE.truncate() * TILE_SIZE;

    if tilemap_size_px.x >= scaled_vp_size.x {
        let (tilemap_left_px, tilemap_right_px) = (-tilemap_size_px.x / 2., tilemap_size_px.x / 2.);
        primary_cam_xform.translation.x = primary_cam_xform.translation.x.clamp(
            tilemap_left_px + scaled_vp_size.x / 2.,
            tilemap_right_px - scaled_vp_size.x / 2.,
        );
    }
    if tilemap_size_px.y >= scaled_vp_size.y {
        let (tilemap_top_px, tilemap_bottom_px) = (tilemap_size_px.y / 2., -tilemap_size_px.y / 2.);
        primary_cam_xform.translation.y = primary_cam_xform.translation.y.clamp(
            tilemap_bottom_px + scaled_vp_size.y / 2.,
            tilemap_top_px - scaled_vp_size.y / 2.,
        );
    }
}

pub fn primary_camera_plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::srgb_u8(208, 187, 148)))
        .add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn((PrimaryCamera, Camera2dBundle::default()));
        })
        .add_systems(
            FixedPostUpdate,
            (follow_player, clamp_to_tilemap)
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
}
