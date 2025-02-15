use std::time::Duration;

use bevy::prelude::*;

use crate::Screen;
use crate::ui_elements;
use crate::despawn_by_tag;


#[derive(Component, Default)]
struct Tag;


pub fn plugin(app: &mut App) {
  app
    .add_systems(OnEnter(Screen::Splash), (
      spawn_title,
//      spawn_sub_title,
    ).chain())

//    .add_systems(Update, (
//      move_title,
//      move_sub_title,
//      hide_title,
//      hide_subtitle,
//      proceed_to_next_screen,
//    ).run_if(
//      in_state(Screen::Splash)
//    ))

    .add_systems(OnExit(Screen::Splash), (
      despawn_by_tag::<Tag>
    ).chain());
}

fn spawn_title(
  mut commands: Commands,
) {
  commands.spawn(TextBundle::from_section(
    "Tennis for Two",
    TextStyle {
      font_size: 60.,
      ..default()
    },
  ).with_style(Style {
    left: Val::Px(0.),
    top: Val::Px(-40.),
    width: Val::Px(40.),
    ..default()
  }));
}
