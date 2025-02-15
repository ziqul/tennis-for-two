use std::time::Duration;

use bevy::prelude::*;

use crate::Screen;
use crate::ui_elements;
use crate::despawn_by_tag;


#[derive(Component, Default)]
struct Tag;
#[derive(Component)]
struct TurnOnVisibilityTimer(Timer);
#[derive(Component)]
struct TurnOffVisibilityTimer(Timer);
#[derive(Resource)]
struct NextStateTimer(Timer);


pub fn plugin(app: &mut App) {
  app
    .insert_resource(NextStateTimer(
      Timer::new(
        Duration::from_secs_f32(6.0),
        TimerMode::Once
      )
    ))

    .add_systems(OnEnter(Screen::Splash), (
      spawn_initial_entities,
    ).chain())

    .add_systems(Update, (
      check_turn_on_visibility,
      check_turn_off_visibility,
      check_proceed_to_next_screen,
    ).run_if(
      in_state(Screen::Splash)
    ))

    .add_systems(
      OnExit(Screen::Splash),
      despawn_by_tag::<Tag>
    );
}


fn spawn_initial_entities(
  mut commands: Commands,
) {
  commands.spawn((
    Tag,
    ui_elements::root::get_element_structure(),
  )).with_children(|parent| {
    parent.spawn(
      ui_elements::title_container::get_element_structure()
    ).with_children(|parent| {
      parent.spawn((
        TurnOnVisibilityTimer(
          Timer::new(
            Duration::from_secs_f32(2.0),
            TimerMode::Once
          )
        ),
        TurnOffVisibilityTimer(
          Timer::new(
            Duration::from_secs_f32(5.0),
            TimerMode::Once
          )
        ),
        set_visibility_to_hidden(
          ui_elements::title::get_element_structure()
        )
      ));
      parent.spawn((
        TurnOnVisibilityTimer(
          Timer::new(
            Duration::from_secs_f32(3.5),
            TimerMode::Once
          )
        ),
        TurnOffVisibilityTimer(
          Timer::new(
            Duration::from_secs_f32(5.0),
            TimerMode::Once
          )
        ),
        set_visibility_to_hidden(
          ui_elements::sub_title::get_element_structure()
        )
      ));
    });
  });
}

fn set_visibility_to_hidden(
  mut node_bundle: TextBundle
) -> TextBundle {
  node_bundle.visibility = Visibility::Hidden;
  return node_bundle;
}

fn check_turn_on_visibility(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut TurnOnVisibilityTimer, &mut Visibility)>,
  asset_server: Res<AssetServer>,
) {
  for (mut timer, mut visibility) in &mut query {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
      *visibility = Visibility::Visible;

      commands.spawn((
        Tag,
        AudioBundle {
          source: asset_server.load("audio/Pause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
      ));
    }
  }
}

fn check_turn_off_visibility(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut TurnOffVisibilityTimer, &mut Visibility)>,
  asset_server: Res<AssetServer>,
) {
  for (mut timer, mut visibility) in &mut query {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
      *visibility = Visibility::Hidden;

      commands.spawn((
        Tag,
        AudioBundle {
          source: asset_server.load("audio/Unpause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
      ));
    }
  }
}

fn check_proceed_to_next_screen(
  time: Res<Time>,
  mut next_state_timer: ResMut<NextStateTimer>,
  mut next_state: ResMut<NextState<Screen>>
) {
  next_state_timer.0.tick(time.delta());

  if next_state_timer.0.just_finished() {
    next_state.set(Screen::Title);
  }
}
