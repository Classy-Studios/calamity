mod asset_owner;
mod game_state;
mod level;
mod mouse_position;
mod player;
mod primary_camera;
mod tile;

use {
    bevy::prelude::*, bevy_rapier2d::prelude::*, game_state::GameState,
    leafwing_input_manager::prelude::*, player::PlayerAction, tile::TILE_SIZE,
};

fn main() {
    App::new()
        .insert_resource({
            let mut rapier_cfg = RapierConfiguration::new((TILE_SIZE.x + TILE_SIZE.y) / 2.);
            rapier_cfg.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_cfg
        })
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter((TILE_SIZE.x + TILE_SIZE.y) / 2.)
                .in_fixed_schedule(),
            //RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            game_state::game_state_plugin,
            level::level_plugin,
            primary_camera::primary_camera_plugin,
            mouse_position::mouse_position_plugin,
            player::player_plugin,
            tile::tile_plugin,
        ))
        .run();
}
