use std::time::Duration;

use belly::prelude::*;
use bevy::prelude::*;

use crate::consts;
use crate::GameState;
use crate::utils::despawn_by_tag;


#[derive(Component, Default)]
struct Tag;
#[derive(Component, Default)]
struct InitHidden;
#[derive(Component)]
struct FadeInTimer(Timer);
#[derive(Component)]
struct FadeOutTimer(Timer);
#[derive(Resource)]
struct NextStateTimer(Timer);


pub fn plugin(app: &mut App) {
  app
    .insert_resource(NextStateTimer(
      Timer::new(Duration::from_secs_f32(6.0), TimerMode::Once)))

    .add_systems(OnEnter(GameState::Splash), (
        spawn_initial_entities,
        init_hidden
    ).chain())

    .add_systems(Update, (
      check_fade_in,
      check_fade_out,
      check_next_state,
    ).run_if(
      in_state(GameState::Splash)
    ))

    .add_systems(OnExit(GameState::Splash), despawn_by_tag::<Tag>);
}

fn spawn_initial_entities(
  mut commands: Commands,
) {
  let title_fade_in = FadeInTimer(Timer::new(Duration::from_secs_f32(2.0), TimerMode::Once));
  let title_fade_out = FadeOutTimer(Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once));
  let author_fade_in = FadeInTimer(Timer::new(Duration::from_secs_f32(3.5), TimerMode::Once));
  let author_fade_out = FadeOutTimer(Timer::new(Duration::from_secs_f32(5.0), TimerMode::Once));

  commands.add(eml! {
    // UI Root
    <div
      with=Tag
      s:flex-direction="column"
      s:align-items="center"
      s:justify-content="center"
      s:width="100%"
      s:height="100%"
    >
      // Game Title
      <span
        with=(InitHidden, title_fade_in, title_fade_out)
        s:visibility="hidden"
        s:font-size="80px"
        s:color="#ffffff"
      >
        "Tennis for Two"
      </span>

      // Author Name
      <span
        with=(InitHidden, author_fade_in, author_fade_out)
        s:visibility="hidden"
        s:font-size="50px"
        s:color="#999999"
      >
        "by @just_soroka"
      </span>

    </div>
  });
}

// Because Belly does not support "s:visibility" style
// prperty we need to hide entities by hand.
fn init_hidden(
  mut query: Query<&mut Visibility, With<InitHidden>>
) {
  for mut visibility in  &mut query {
    *visibility = Visibility::Hidden;
  }
}

fn check_fade_in(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut FadeInTimer, &mut Visibility)>,
  asset_server: Res<AssetServer>,
) {
  for (mut timer, mut visibility) in &mut query {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
      println!("awd");
      *visibility = Visibility::Visible;
      commands.spawn((
        AudioBundle {
          source: asset_server.load("audio/Pause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
        Tag
      ));
    }
  }
}

fn check_fade_out(
  mut commands: Commands,
  time: Res<Time>,
  mut query: Query<(&mut FadeOutTimer, &mut Visibility)>,
  asset_server: Res<AssetServer>,
) {
  for (mut timer, mut visibility) in &mut query {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
      *visibility = Visibility::Hidden;
      commands.spawn((
        AudioBundle {
          source: asset_server.load("audio/Unpause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
        Tag
      ));
    }
  }
}

fn check_next_state(
  time: Res<Time>,
  mut next_state_timer: ResMut<NextStateTimer>,
  mut next_state: ResMut<NextState<GameState>>
) {
  next_state_timer.0.tick(time.delta());

  if next_state_timer.0.just_finished() {
    next_state.set(GameState::Title);
  }
}
