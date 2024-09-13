use {
    super::{
        asset_owner::TextureAtlasOwner, game_state::GameState, level::LEVEL_SIZE, player::Player,
    },
    bevy::prelude::*,
    bevy_rapier2d::prelude::*,
    rand::Rng,
};

#[derive(Component)]
pub struct Zombie;

#[derive(Resource)]
pub struct ZombieSpawns(pub Vec<Vec2>);

impl ZombieSpawns {
    fn new() -> Self {
        Self(Vec::with_capacity(
            LEVEL_SIZE.x as usize * 2 + LEVEL_SIZE.y as usize * 2 - 4,
        ))
    }
}

fn spawn_zombie(
    mut cmds: Commands,
    time: Res<Time>,
    zombie_spawns: Res<ZombieSpawns>,
    zombie_tex_atlas: Res<TextureAtlasOwner<Zombie>>,
) {
    let t = time.elapsed_seconds() as u32;
    if t % 15 == 0 {
        let mut thread_rng = rand::thread_rng();
        for _ in 0..(t / 15) {
            cmds.spawn((
                StateScoped(GameState::Playing),
                Zombie,
                Collider::ball(15.),
                SpriteBundle {
                    transform: Transform::from_translation(
                        zombie_spawns.0[thread_rng.gen_range(0..zombie_spawns.0.len())].extend(8.),
                    ),
                    texture: zombie_tex_atlas.texture(),
                    ..default()
                },
                TextureAtlas {
                    index: 0,
                    layout: zombie_tex_atlas.layout(),
                },
                KinematicCharacterController::default(),
                Velocity::linear(Vec2::splat(15.)),
            ));
        }
    }
}

fn zombie_movement(
    mut zombie_qry: Query<(&mut KinematicCharacterController, &mut Transform, &Velocity), With<Zombie>>,
    player_qry: Query<&Transform, (With<Player>, Without<Zombie>)>,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    let player_xform = player_qry.single();
    for (mut zombie_kcc, mut zombie_xform, zombie_vel) in &mut zombie_qry {
        let zombie_dir = -(zombie_xform.translation.truncate() - player_xform.translation.truncate()).normalize();
        let theta = -zombie_dir.angle_between(Vec2::X);
        zombie_xform.rotation = Quat::from_rotation_z(theta);
        let zombie_displacement = zombie_dir * zombie_vel.linvel * dt;
        zombie_kcc.translation = Some(zombie_displacement);
    }
}

pub fn zombie_plugin(app: &mut App) {
    app.insert_resource(ZombieSpawns::new())
        .add_systems(
            OnEnter(GameState::Setup),
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(TextureAtlasOwner::<Zombie>::new(
                    asset_server.load("zombie.png"),
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
        .add_systems(Update, spawn_zombie.run_if(in_state(GameState::Playing)))
        .add_systems(
            FixedUpdate,
            zombie_movement.run_if(in_state(GameState::Playing)),
        );
}
