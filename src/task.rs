use {
    super::{
        game_state::GameState,
        level::{self, LevelObject, LEVEL_LAYOUT, LEVEL_SIZE},
        mouse_position::MousePosition,
        player::{Player, PlayerHealthBar},
        tile::AVG_TILE_DIMENSION,
        player,
    },
    bevy::prelude::*,
    rand::Rng,
    std::collections::VecDeque,
    strum::EnumCount,
    strum_macros::EnumCount as EnumCountMacro,
};

#[derive(Component, EnumCountMacro, PartialEq, Eq, Clone, Copy)]
pub enum Task {
    PatchLeak,
    ExtinguishFire,
    PowerGenerator,
    BoardWindow,
}

impl Task {
    pub fn name(&self) -> &str {
        match self {
            Self::PatchLeak => "Patch leak",
            Self::ExtinguishFire => "Extinguish fire",
            Self::PowerGenerator => "Power generator",
            Self::BoardWindow => "Board window",
        }
    }

    pub fn level_object(&self) -> LevelObject {
        match self {
            Self::BoardWindow => LevelObject::ShatteredGlass,
            _ => LevelObject::ShatteredGlass,
        }
    }

    pub fn pos(&self) -> Vec3 {
        for z in 0..LEVEL_SIZE.z as usize {
            for y in 0..LEVEL_SIZE.y as usize {
                for x in 0..LEVEL_SIZE.x as usize {
                    if self.level_object() == LEVEL_LAYOUT[z][y][x] {
                        return level::idx_to_world_pos(x, y, z);
                    }
                }
            }
        }
        Vec3::ZERO
    }
}

impl TryFrom<usize> for Task {
    type Error = ();

    fn try_from(val: usize) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::PatchLeak),
            1 => Ok(Self::ExtinguishFire),
            2 => Ok(Self::PowerGenerator),
            3 => Ok(Self::BoardWindow),
            Self::COUNT.. => Err(()),
        }
    }
}

#[derive(Resource)]
pub struct TaskList(VecDeque<Entity>);

impl TaskList {
    pub const MAX_SIZE: usize = 3;

    fn new() -> Self {
        Self(VecDeque::with_capacity(Self::MAX_SIZE))
    }

    pub fn get(&self, idx: usize) -> Option<&Entity> {
        self.0.get(idx)
    }
}

#[derive(Component)]
pub struct TaskTimer(Timer);

impl TaskTimer {
    pub fn remaining_secs(&self) -> f32 {
        self.0.remaining_secs()
    }
}

fn spawn_task(mut cmds: Commands, mut task_list: ResMut<TaskList>, task_qry: Query<&Task>) {
    if task_list.0.len() != TaskList::MAX_SIZE {
        let mut task;
        while {
            task = Task::try_from(rand::thread_rng().gen_range(0..Task::COUNT)).unwrap();
            task_qry.iter().any(|&other_task| other_task == task)
        } {}
        task_list.0.push_back(
            cmds.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(64.)),
                        ..default()
                    },
                    transform: Transform::from_translation(task.pos()),
                    ..default()
                },
                task,
                TaskTimer(Timer::from_seconds(60., TimerMode::Once)),
            ))
            .id(),
        );
    }
}

pub fn update_task_timers(
    mut cmds: Commands,
    mut task_timer_qry: Query<(Entity, &mut TaskTimer), With<Task>>,
    mut task_list: ResMut<TaskList>,
    mut player_hp_bar: ResMut<PlayerHealthBar>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for (task_id, mut task_timer) in &mut task_timer_qry {
        task_timer.0.tick(dt);
        if task_timer.0.just_finished() {
            cmds.entity(task_id).despawn_recursive();
            task_list.0.pop_front();
            player_hp_bar.0.pop();
        }
    }
}

fn mouse_highlight_task(
    mouse_pos: Res<MousePosition>,
    mut task_qry: Query<(&mut Sprite, &Transform), With<Task>>,
    player_qry: Query<&Transform, With<Player>>,
) {
    let player_pos = player_qry.single().translation.truncate();
    let Some(mut closest_task_sprite) = task_qry
        .iter_mut()
        .map(|(task_sprite, task_xform)| (task_sprite, task_xform.translation.truncate()))
        .find(|(_, task_pos)| {
            (mouse_pos.as_vec().distance(*task_pos) <= AVG_TILE_DIMENSION)
                && (player_pos.distance(*task_pos) <= AVG_TILE_DIMENSION)
        })
        .map(|(task_sprite, _)| task_sprite)
    else {
        for (mut task_sprite, _) in &mut task_qry {
            task_sprite.color = Color::Srgba(Srgba::WHITE);
        }
        return;
    };

    closest_task_sprite.color = Color::Srgba(Srgba::rgba_u8(255, 255, 0, 150));
}

pub fn task_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (update_task_timers, mouse_highlight_task).run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        OnEnter(GameState::Playing),
        (
            |mut cmds: Commands| {
                cmds.insert_resource(TaskList::new());
            },
            spawn_task,
        )
            .chain(),
    );
}
