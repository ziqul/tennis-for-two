use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::text::Text;

use super::GameState;
use super::despawn_by_tag;
use super::create_ui_root;


#[derive(Component)]
struct Tag;
#[derive(Component)]
struct FadeIn {
  delay_sec: f32,
  executed: bool,
}
#[derive(Component)]
struct FadeOut {
  delay_sec: f32,
  executed: bool,
}
#[derive(Component)]
struct NextStateTimer {
  delay_sec: f32,
}
#[derive(Resource)]
struct MainStopwatch(Stopwatch);


pub fn plugin(app: &mut App) {
  app
    .add_systems(OnEnter(GameState::Splash), (
      add_splash_ui_elements,
      add_next_state_timer,
    ))
    .insert_resource(MainStopwatch(Stopwatch::new()))
    .add_systems(Update, (
      fade_in,
      fade_out,
      next_state_check,
      tick_stopwatch,
    ).run_if(in_state(GameState::Splash)))
    .add_systems(OnExit(GameState::Splash), despawn_by_tag::<Tag>);
}

fn add_next_state_timer(mut commands: Commands) {
  commands
    .spawn((
      NextStateTimer { delay_sec: 6.0 },
      Tag
    ));
}

fn add_splash_ui_elements(mut commands: Commands) {
  commands
    .spawn((
      create_ui_root(),
      Tag,
    ))
    .with_children(|parent| {
      parent.spawn((
        TextBundle{
          text: Text::from_section(
            "Tennis for Two",
            TextStyle {
              font_size: 80.0,
              color: Color::srgb(1.0, 1.0, 1.0),
              ..default()
            }
          ),
          visibility: Visibility::Hidden,
          ..default()
        },
        FadeIn { delay_sec: 2.0, executed: false },
        FadeOut { delay_sec: 5.0, executed: false },
      ));
    })
    .with_children(|parent| {
      parent.spawn((
        TextBundle{
          text: Text::from_section(
            "by @just_soroka",
            TextStyle {
              font_size: 50.0,
              color: Color::srgb(0.33, 0.33, 0.33),
              ..default()
            }
          ),
          visibility: Visibility::Hidden,
          ..default()
        },
        FadeIn { delay_sec: 3.5, executed: false },
        FadeOut { delay_sec: 5.0, executed: false },
      ));
    })
    ;
}

fn fade_in(
  mut commands: Commands,
  mut query_fade_in_elements: Query<(&mut Visibility, &mut FadeIn), With<Text>>,
  stopwatch: Res<MainStopwatch>,
  asset_server: Res<AssetServer>,
) {
  for (mut text_visibility, mut fade_in) in &mut query_fade_in_elements {
    if
      stopwatch.0.elapsed_secs() >= fade_in.delay_sec &&
      fade_in.executed == false
    {
      *text_visibility = Visibility::Visible;
      commands.spawn((
        AudioBundle {
          source: asset_server.load("audio/Pause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
        Tag
      ));
      fade_in.executed = true;
    }
  }
}

fn fade_out(
  mut commands: Commands,
  mut query_fade_in_elements: Query<(&mut Visibility, &mut FadeOut), With<Text>>,
  stopwatch: Res<MainStopwatch>,
  asset_server: Res<AssetServer>,
) {
  for (mut text_visibility, mut fade_out) in &mut query_fade_in_elements {
    if
      stopwatch.0.elapsed_secs() >= fade_out.delay_sec &&
      fade_out.executed == false
    {
      *text_visibility = Visibility::Hidden;
      commands.spawn((
        AudioBundle {
          source: asset_server.load("audio/Unpause.wav"),
          settings: PlaybackSettings::ONCE,
          ..default()
        },
        Tag
      ));
      fade_out.executed = true;
    }
  }
}

fn next_state_check(
  stopwatch: Res<MainStopwatch>,
  query: Query<&NextStateTimer>,
  mut next_state: ResMut<NextState<GameState>>
) {
  for timer in &query {
    if stopwatch.0.elapsed_secs() >= timer.delay_sec {
      next_state.set(GameState::Exit);
    }
  }
}

fn tick_stopwatch(
  time: Res<Time>,
  mut stopwatch: ResMut<MainStopwatch>,
) {
  stopwatch.0.tick(time.delta());
}
