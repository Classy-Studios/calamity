use {
    super::{
        asset_owner::FontOwner,
        asset_owner::TextureAtlasOwner,
        level,
        player::PlayerHealthBar,
        task::{self, Task, TaskList, TaskTimer},
        GameState,
    },
    crate::RESOLUTION,
    bevy::prelude::*,
};

#[derive(Component)]
struct Ui;

#[derive(Component)]
struct TaskUi;

#[derive(Component)]
struct HealthUi;

fn spawn_hud(mut cmds: Commands, health_bar_tex_atlas: Res<TextureAtlasOwner<HealthUi>>) {
    cmds.spawn((
        Ui,
        StateScoped(GameState::Playing),
        NodeBundle {
            style: Style {
                width: Val::Px(RESOLUTION.x),
                height: Val::Px(RESOLUTION.y),
                ..default()
            },
            ..default()
        },
    ))
    .with_children(|screen| {
        screen
            .spawn(NodeBundle {
                style: Style {
                    height: Val::Percent(25.),
                    width: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            })
            .with_children(|hud| {
                hud.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(40.),
                        height: Val::Percent(30.),
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,

                        ..default()
                    },
                    ..default()
                })
                .with_children(|health_bar| {
                    for _ in 0..PlayerHealthBar::MAX_SIZE {
                        health_bar.spawn((
                            HealthUi,
                            ImageBundle {
                                style: Style {
                                    width: Val::Percent(10.),
                                    height: Val::Percent(100.),
                                    ..default()
                                },
                                image: UiImage {
                                    texture: health_bar_tex_atlas.texture(),
                                    ..default()
                                },
                                ..default()
                            },
                            TextureAtlas {
                                layout: health_bar_tex_atlas.layout(),
                                index: 0,
                            },
                        ));
                    }
                });
                hud.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(30.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|task_list| {
                    for _ in 0..TaskList::MAX_SIZE {
                        task_list
                            .spawn(NodeBundle {
                                style: Style {
                                    height: Val::Percent(100. / 4.),
                                    width: Val::Percent(90.),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                border_radius: BorderRadius::all(Val::Percent(30.)),
                                background_color: BackgroundColor(Color::Srgba(Srgba::rgba_u8(
                                    120, 120, 120, 150,
                                ))),
                                ..default()
                            })
                            .with_children(|task| {
                                task.spawn((
                                    TaskUi,
                                    TextBundle::from_section("", TextStyle::default()),
                                ));
                            });
                    }
                });
            });
    });
}

fn update_hud(
    task_list: Res<TaskList>,
    task_qry: Query<(&Task, &TaskTimer)>,
    mut task_ui_qry: Query<&mut Text, With<TaskUi>>,
    player_hp_bar: Res<PlayerHealthBar>,
    mut hp_ui_qry: Query<&mut TextureAtlas, With<HealthUi>>,
) {
    for (i, mut task_ui) in task_ui_qry.iter_mut().enumerate() {
        task_ui.sections[0] = task_list
            .get(i)
            .and_then(|&task_id| task_qry.get(task_id).ok())
            .map(|(task, task_timer)| {
                format!("{}! {:.0} s", task.name(), task_timer.remaining_secs())
            })
            .unwrap_or(String::new())
            .into();
    }
    for (i, mut hp_ui) in hp_ui_qry.iter_mut().enumerate() {
        hp_ui.index = match player_hp_bar.0.get(i) {
            Some(2) => 0,
            Some(1) => 1,
            _ => 2,
        }
    }
}

pub fn ui_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Setup),
        (
            |mut cmds: Commands, asset_server: Res<AssetServer>| {
                cmds.insert_resource(FontOwner::<Ui>::new(asset_server.load("font.ttf")));
            },
            |mut cmds: Commands,
             asset_server: Res<AssetServer>,
             mut tex_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>| {
                cmds.insert_resource(TextureAtlasOwner::<HealthUi>::new(
                    asset_server.load("health.png"),
                    tex_atlas_layouts.add(TextureAtlasLayout::from_grid(
                        UVec2::splat(128),
                        1,
                        3,
                        None,
                        None,
                    )),
                ))
            },
        ),
    )
    .add_systems(
        OnEnter(GameState::Playing),
        spawn_hud.after(level::spawn_level_objects),
    )
    .add_systems(
        Update,
        update_hud
            .after(task::update_task_timers)
            .run_if(in_state(GameState::Playing)),
    );
}
