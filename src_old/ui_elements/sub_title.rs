use bevy::prelude::*;

pub fn get_element_structure() -> TextBundle {
  TextBundle::from_section(
    "by @just_soroka",
    TextStyle {
      font_size: 40.,
      ..default()
    },
  )
}
