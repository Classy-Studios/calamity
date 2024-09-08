use {super::asset_owner::TextureAtlasOwner, crate::GameState, bevy::prelude::*};

pub const TILE_SIZE: Vec2 = Vec2::splat(64.);

#[derive(Component)]
struct Tile;

pub fn tile_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Setup),
        |mut cmds: Commands,
         asset_server: Res<AssetServer>,
         mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
            cmds.insert_resource(TextureAtlasOwner::<Tile>::new(
                asset_server.load("tile.png"),
                tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                    UVec2::new(TILE_SIZE.x as u32, TILE_SIZE.y as u32),
                    27,
                    20,
                    None,
                    None,
                )),
            ))
        },
    );
}
