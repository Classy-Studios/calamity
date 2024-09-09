use {
    super::{asset_owner::TextureAtlasOwner, GameState},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const TILE_SIZE: Vec2 = Vec2::splat(64.);

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum TileIndex {
    Grass1 = 0,
    Grass2 = 1,
    Grass3 = 2,
    Grass4 = 3,
    Dirt1 = 4,
    Dirt2 = 5,
    Gravel1 = 6,
    Gravel2 = 7,
    Gravel3 = 8,
    Gravel4 = 9,
    Water1 = 18,
    Water2 = 19,
    Floor1 = 95,
    Floor2 = 96,
    BrokenFloor1 = 98,
    BrokenFloor2 = 99,
    BrokenFloor3 = 100,
    SinglePillow = 101,
    SingleMiddle = 74,
    SingleFoot = 47,
    DoublePillow1 = 102,
    DoublePillow2 = 103,
    DoubleMiddle1 = 75,
    DoubleMiddle2 = 76,
    DoubleFoot1 = 48,
    DoubleFoot2 = 49,
    WallTopLeft = 124,
    WallTopRight = 125,
    WallBottomLeft = 151,
    WallBottomRight = 152,
    WallLeft = 175,
    WallRight = 176,
    WallTop = 202,
    WallBottom = 203,
    Stove = 323,
    Sink = 322,
    BigTv1 = 531,
    BigTv2 = 532,
    SmallTv = 536,
    Couch1 = 500,
    Couch2 = 501,
    Couch3 = 502,
    Recliner = 504,
    BedsideTable = 478,
    CoffeeTable = 505,
    Chair = 477,
    ShatteredGlass1 = 263,
    ShatteredGlass2 = 290,
    WoodPlank = 264,
    Debris1 = 291,
    Debris2 = 292,
    OilSpill = 319,
    KitchenTop = 320,
    TreeTopLeft = 180,
    TreeTopRight = 181,
    TreeBottomLeft = 207,
    TreeBottomRight = 208,
    Bush1 = 182,
    Bush2 = 209,
    FallenLeaves = 212,
    Box1 = 128,
    Box2 = 129,
    Box3 = 155,
    Box4 = 156,
    UWall = 123,
    Marble = 11,
    CornerTopLeft = 199,
    CornerTopRight = 198,
    CornerBottomRight = 171,
    CornerBottomLeft = 172,
}

#[derive(Component)]
pub struct Tile;

pub fn spawn_tile(
    cmds: &mut Commands,
    tile_pos: Vec2,
    tile_tex_atlas: &Res<TextureAtlasOwner<Tile>>,
    tile_tex_idx: usize,
) {
    cmds.spawn((
        Tile,
        SpriteBundle {
            transform: Transform::from_translation(tile_pos.extend(1.)),
            texture: tile_tex_atlas.texture(),
            ..default()
        },
        TextureAtlas {
            layout: tile_tex_atlas.layout(),
            index: tile_tex_idx,
        },
        //Collider::cuboid(TILE_SIZE.x / 2., TILE_SIZE.y / 2.),
    ));
}

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
