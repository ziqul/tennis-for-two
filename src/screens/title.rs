use std::time::Duration;

use belly::prelude::*;
use bevy::prelude::*;
use bevy::audio::Volume;
use bevy::audio::PlaybackMode;
use bevy::audio::PlaybackSettings;

use crate::menus;
use crate::GameState;
use crate::utils::despawn_by_tag;


#[derive(Component, Default)]
struct Tag;


pub fn plugin(app: &mut App) {
  app
    //.insert_resource(NextStateTimer(
    //  Timer::new(Duration::from_secs_f32(6.0), TimerMode::Once)))

    .add_systems(OnEnter(GameState::Title), (
        draw_title_ui,
        setup,
        //start_title_screen_music,
    ).chain())

    .add_systems(Update, (
        test,
    ).run_if(
      in_state(GameState::Title)
    ))

    .add_systems(OnExit(GameState::Title), despawn_by_tag::<Tag>);
}

fn start_title_screen_music(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  commands.spawn((
    AudioBundle {
      source: asset_server.load("audio/Zoolook.mp3"),
      settings: PlaybackSettings {
        mode: PlaybackMode::Loop,
        volume: Volume::new(0.3),
        speed: 0.9,
        ..default()
      }
    },
    Tag
  ));
}

fn draw_title_ui(
  mut commands: Commands,
) {
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
        s:font-size="80px"
        s:color="#ffffff"
      >
        "Tennis for Two"
      </span>

      // Menu Container
      <div
        id="menu-container"
        s:height="500px"
        s:align-items="center"
        s:justify-content="center"
      >
      </div>

    </div>
  });
}

fn setup(
  mut elements: Elements,
  mut commands: Commands,
) {
  let mut menu_container = elements.select("#menu-container");
  menu_container.add_child(menus::main::get_nodes(&mut commands));
}

fn test(
  mut elements: Elements,
  keys: Res<ButtonInput<KeyCode>>,
) {
  //if keys.just_pressed(KeyCode::Space) {
  //  let menu_container_children = elements.select("#menu-container > *");
  //  menu_container_children.remove();
  //}
}
