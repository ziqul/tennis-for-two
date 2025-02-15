use bevy::prelude::*;

pub fn get_element_structure(text: &str) -> TextBundle {
  TextBundle::from_section(
    text,
    TextStyle {
      font_size: 50.,
      ..default()
    },
  )
}
