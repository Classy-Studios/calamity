use {
    super::GameState, bevy::prelude::*, rand::Rng, std::collections::VecDeque, strum::EnumCount,
    strum_macros::EnumCount as EnumCountMacro,
};

pub const MAX_TASK_COUNT: usize = 3;

#[derive(Component, EnumCountMacro)]
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

fn spawn_task(mut cmds: Commands, mut task_list: ResMut<TaskList>) {
    if task_list.0.len() != MAX_TASK_COUNT {
        task_list.0.push_back(
            cmds.spawn((
                Task::try_from(rand::thread_rng().gen_range(0..Task::COUNT)).unwrap(),
                TaskTimer(Timer::from_second(60., TimerMode::Once)),
            ))
            .id(),
        );
    }
}

pub fn update_task_timers(
    mut cmds: Commands,
    mut task_timer_qry: Query<(Entity, &mut TaskTimer), With<Task>>,
    mut task_list: ResMut<TaskList>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for (task_id, mut task_timer) in &mut task_timer_qry {
        task_timer.0.tick(dt);
        if task_timer.0.just_finished() {
            cmds.entity(task_id).despawn_recursive();
            task_list.0.pop_front();
        }
    }
}

pub fn task_plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_task_timers.run_if(in_state(GameState::Playing)),
    )
    .add_systems(
        OnEnter(GameState::Playing),
        (
            |mut cmds: Commands| {
                cmds.insert_resource(TaskList(VecDeque::with_capacity(MAX_TASK_COUNT)));
            },
            spawn_task,
        )
            .chain(),
    );
}
