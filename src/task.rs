use {
    super::GameState, bevy::prelude::*, rand::Rng, strum::EnumCount,
    strum_macros::EnumCount as EnumCountMacro,
};

#[derive(Component, EnumCountMacro)]
pub enum Task {
    PatchLeak,
    ExtinguishFire,
    PowerGenerator,
    BoardWindow,
}

impl Task {
    fn name(&self) -> Name {
        match self {
            Self::PatchLeak => Name::new("Patch leak"),
            Self::ExtinguishFire => Name::new("Extinguish fire"),
            Self::PowerGenerator => Name::new("Power generator"),
            Self::BoardWindow => Name::new("Board window"),
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

#[derive(Component)]
pub struct TaskTimer(Timer);

impl TaskTimer {
    pub fn remaining_secs(&self) -> f32 {
        self.0.remaining_secs()
    }
}

fn spawn_task(mut cmds: Commands) {
    let task = Task::try_from(rand::thread_rng().gen_range(0..Task::COUNT)).unwrap();
    cmds.spawn((
        task.name(),
        task,
        TaskTimer(Timer::from_seconds(60., TimerMode::Once)),
    ));
}

pub fn update_task_timers(mut task_timer_qry: Query<&mut TaskTimer, With<Task>>, time: Res<Time>) {
    let dt = time.delta();
    for mut task_timer in &mut task_timer_qry {
        task_timer.0.tick(dt);
    }
}

pub fn task_plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_task_timers.run_if(in_state(GameState::Playing)),
    )
    .add_systems(OnEnter(GameState::Playing), spawn_task);
}
