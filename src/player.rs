use {
    super::{
        asset_owner::TextureAtlasOwner,
        game_state::GameState,
        mouse_position::MousePosition,
        task::*,
        tile::{AVG_TILE_DIMENSION, TILE_SIZE},
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    leafwing_input_manager::prelude::*,
};

#[derive(Component)]
pub struct Player {
    doing_task: bool,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Reflect, Debug)]
pub enum PlayerAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    EnterTask,
    ExitTask,
}

#[derive(Resource)]
pub struct PlayerHealthBar(pub Vec<u8>);

impl PlayerHealthBar {
    pub const MAX_SIZE: usize = 3;

    fn new() -> Self {
        Self(vec![2; Self::MAX_SIZE])
    }
}

pub fn spawn_player(
    cmds: &mut Commands,
    player_pos: Vec2,
    player_tex_atlas: &Res<TextureAtlasOwner<Player>>,
) {
    cmds.spawn((
        Player { doing_task: false },
        StateScoped(GameState::Playing),
        SpriteBundle {
            texture: player_tex_atlas.texture(),
            transform: Transform::from_translation(player_pos.extend(10.)),
            ..default()
        },
        TextureAtlas {
            layout: player_tex_atlas.layout(),
            index: 0,
        },
        KinematicCharacterController::default(),
        Collider::ball(15.),
        InputManagerBundle::with_map(
            InputMap::new([
                (PlayerAction::MoveLeft, KeyCode::KeyA),
                (PlayerAction::MoveRight, KeyCode::KeyD),
                (PlayerAction::MoveUp, KeyCode::KeyW),
                (PlayerAction::MoveDown, KeyCode::KeyS),
                (PlayerAction::ExitTask, KeyCode::Escape),
            ])
            .with(PlayerAction::EnterTask, MouseButton::Left),
        ),
        Velocity::linear(TILE_SIZE * 2.),
    ));
}

//If player is within proximity to a task, when left mouse is clicked enter that task
fn player_task_input(
    mut player_qry: Query<(&mut Player, &Transform, &ActionState<PlayerAction>)>,
    task_qry: Query<&Transform, With<Task>>,
) {
    let (mut player, player_xform, player_in) = player_qry.single_mut();

    if player.doing_task {
        if player_in.just_pressed(&PlayerAction::ExitTask) {
            player.doing_task = false;
        }
        return;
    }
    let player_pos = player_xform.translation.truncate();

    let Some(closest_task_pos) = task_qry
        .iter()
        .map(|task_xform| task_xform.translation.truncate())
        .min_by(|task_a_pos, task_b_pos| {
            (task_a_pos.distance(player_pos)).total_cmp(&task_b_pos.distance(player_pos))
        })
    else {
        return;
    };

    if player_in.just_pressed(&PlayerAction::EnterTask)
        && closest_task_pos.distance(player_pos) <= AVG_TILE_DIMENSION
    {
        player.doing_task = true;
        println!("doing task!!!");
    }
}

fn player_movement(
    mut player_qry: Query<(
        &Player,
        &mut KinematicCharacterController,
        &mut Transform,
        &ActionState<PlayerAction>,
        &Velocity,
    )>,
    mouse_pos: Res<MousePosition>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let (player, mut player_kcc, mut player_xform, player_in, player_vel) = player_qry.single_mut();
    if player.doing_task {
        return;
    };

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
    .add_systems(OnEnter(GameState::Playing), |mut cmds: Commands| {
        cmds.insert_resource(PlayerHealthBar::new())
    })
    .add_systems(
        Update,
        player_task_input.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        FixedUpdate,
        player_movement.run_if(in_state(GameState::Playing)),
    );
}
