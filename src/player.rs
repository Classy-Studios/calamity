use {
    super::asset_owner::TextureAtlasOwner, crate::GameState, bevy::prelude::*,
    leafwing_input_manager::prelude::*,
};

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    Jump,
    DropDown,
    EnterDoor,
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Setup),
        |mut cmds: Commands,
         asset_server: Res<AssetServer>,
         mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
            cmds.insert_resource(TextureAtlasOwner::<Player>::new(
                asset_server.load("player.png"),
                tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(80, 110), // CHANGE
                    9,                   // CHANGE
                    3,                   // CHANGE
                    None,                // CHANGE
                    None,                // CHANGE
                )),
            ))
        },
    );
}
