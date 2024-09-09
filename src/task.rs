use {
    super::{
        asset_owner::TextureAtlasOwner, game_state::GameState, mouse_position::MousePosition,
        tile::TILE_SIZE,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

#[Derive(Compnent)]
pub enum Task {
    PUT_OUT_FIRE,
    FIX_WINDOW,
    PATCH_LEAK,
    FIX_POWER,
}

//Will run logic for task depending on what it is
fn run_task() {

}