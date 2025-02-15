use bevy::prelude::*;

pub fn get_element_structure() -> TextBundle {
  TextBundle::from_section(
    "Tennis for Two",
    TextStyle {
      font_size: 60.,
      ..default()
    },
  )
}
