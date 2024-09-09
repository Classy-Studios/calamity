use {
    super::{asset_owner::TextureAtlasOwner, level::LevelObject, GameState},
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
};

pub const TILE_SIZE: Vec2 = Vec2::splat(64.);
const WALL_THICKNESS: f32 = 15.;
const HALF_WALL_THICKNESS: f32 = WALL_THICKNESS / 2.;
const HALF_TILE_SIZE: f32 = (TILE_SIZE.x + TILE_SIZE.y) / 2. / 2.;

#[derive(Component)]
pub struct Tile;

pub fn spawn_tile(
    cmds: &mut Commands,
    tile_pos: Vec2,
    tile_z: f32,
    tile_tex_atlas: &Res<TextureAtlasOwner<Tile>>,
    tile_tex_idx: usize,
    tile_lvl_obj: LevelObject,
) {
    cmds.spawn((
        Tile,
        SpriteBundle {
            transform: Transform::from_translation(tile_pos.extend(tile_z)),
            texture: tile_tex_atlas.texture(),
            ..default()
        },
        TextureAtlas {
            layout: tile_tex_atlas.layout(),
            index: tile_tex_idx,
        },
    ))
    .with_children(|parent| {
        if matches!(
            tile_lvl_obj,
            LevelObject::WallLeft
                | LevelObject::WallBottomLeft
                | LevelObject::WallTopLeft
                | LevelObject::UWall
        ) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    0.,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_TILE_SIZE),
            ));
        }
        if matches!(
            tile_lvl_obj,
            LevelObject::WallRight
                | LevelObject::WallBottomRight
                | LevelObject::WallTopRight
                | LevelObject::UWall
        ) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    0.,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_TILE_SIZE),
            ));
        }
        if matches!(
            tile_lvl_obj,
            LevelObject::WallTop | LevelObject::WallTopLeft | LevelObject::WallTopRight
        ) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_TILE_SIZE, HALF_WALL_THICKNESS),
            ));
        }
        if matches!(
            tile_lvl_obj,
            LevelObject::WallBottom
                | LevelObject::WallBottomLeft
                | LevelObject::WallBottomRight
                | LevelObject::UWall
        ) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    0.,
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_TILE_SIZE, HALF_WALL_THICKNESS),
            ));
        }
        if matches!(tile_lvl_obj, LevelObject::CornerTopLeft) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_WALL_THICKNESS),
            ));
        }
        if matches!(tile_lvl_obj, LevelObject::CornerTopRight) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_WALL_THICKNESS),
            ));
        }
        if matches!(tile_lvl_obj, LevelObject::CornerBottomLeft) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_WALL_THICKNESS),
            ));
        }
        if matches!(tile_lvl_obj, LevelObject::CornerBottomRight) {
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    HALF_TILE_SIZE + HALF_WALL_THICKNESS - WALL_THICKNESS,
                    -HALF_TILE_SIZE - HALF_WALL_THICKNESS + WALL_THICKNESS,
                    0.,
                )),
                Collider::cuboid(HALF_WALL_THICKNESS, HALF_WALL_THICKNESS),
            ));
        }
    });
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
