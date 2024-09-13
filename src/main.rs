mod asset_owner;
mod game_state;
mod level;
mod mouse_position;
mod player;
mod primary_camera;
mod task;
mod tile;
mod ui;
mod zombie;
mod combat;

use {
    bevy::prelude::*, bevy_rapier2d::prelude::*, game_state::GameState,
    leafwing_input_manager::prelude::*, player::PlayerAction, tile::AVG_TILE_DIMENSION,
};

const RESOLUTION: Vec2 = Vec2::new(1280., 720.);

fn main() {
    App::new()
        .insert_resource({
            let mut rapier_cfg = RapierConfiguration::new(AVG_TILE_DIMENSION);
            rapier_cfg.timestep_mode = TimestepMode::Fixed {
                dt: Time::<Fixed>::default().timestep().as_secs_f32(),
                substeps: 1,
            };
            rapier_cfg
        })
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(AVG_TILE_DIMENSION)
                .in_fixed_schedule(),
            //RapierDebugRenderPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
            game_state::game_state_plugin,
            level::level_plugin,
            primary_camera::primary_camera_plugin,
            mouse_position::mouse_position_plugin,
            ui::ui_plugin,
            task::task_plugin,
            player::player_plugin,
            zombie::zombie_plugin,
            tile::tile_plugin,
        ))
        .run();
}
