use {
    super::{
        asset_owner::TextureAtlasOwner, game_state::GameState, mouse_position::MousePosition,
        tile::TILE_SIZE,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

pub fn spawn_player(
    cmds: &mut Commands,
    player_pos: Vec2,
    player_tex_atlas: &Res<TextureAtlasOwner<Player>>,
) {
    cmds.spawn((
        Player,
        SpriteBundle {
            texture: player_tex_atlas.texture(),
            transform: Transform::from_translation(player_pos.extend(2.)),
            ..default()
        },
        TextureAtlas {
            layout: player_tex_atlas.layout(),
            index: 0,
        },
        KinematicCharacterController::default(),
        Collider::ball(15.),
        InputManagerBundle::with_map(InputMap::new([
            (PlayerAction::MoveLeft, KeyCode::KeyA),
            (PlayerAction::MoveRight, KeyCode::KeyD),
            (PlayerAction::MoveUp, KeyCode::KeyW),
            (PlayerAction::MoveDown, KeyCode::KeyS),
        ])),
        Velocity::linear(TILE_SIZE * 2.),
    ));
}

fn player_movement(
    mut player_qry: Query<
        (
            &mut KinematicCharacterController,
            &mut Transform,
            &ActionState<PlayerAction>,
            &Velocity,
        ),
        With<Player>,
    >,
    mouse_pos: Res<MousePosition>,
    time: Res<Time<Fixed>>,
) {
    let dt = time.delta_seconds();
    let (mut player_kcc, mut player_xform, player_in, player_vel) = player_qry.single_mut();

    let theta = -(mouse_pos.as_vec() - player_xform.translation.truncate()).angle_between(Vec2::X);
    player_xform.rotation = Quat::from_rotation_z(theta);

    let mut displacement = Vec2::ZERO;

    if player_in.pressed(&PlayerAction::MoveLeft) {
        displacement.x -= player_vel.linvel.x * dt;
    }
    if player_in.pressed(&PlayerAction::MoveRight) {
        displacement.x += player_vel.linvel.x * dt;
    }
    if player_in.pressed(&PlayerAction::MoveUp) {
        displacement.y += player_vel.linvel.y * dt;
    }
    if player_in.pressed(&PlayerAction::MoveDown) {
        displacement.y -= player_vel.linvel.y * dt;
    }
    player_kcc.translation = Some(displacement);
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
                    UVec2::splat(64),
                    2,
                    1,
                    None,
                    None,
                )),
            ))
        },
    )
    .add_systems(
        FixedUpdate,
        player_movement.run_if(in_state(GameState::Playing)),
    );
}
