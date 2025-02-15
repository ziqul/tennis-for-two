use bevy::prelude::*;

pub fn get_element_structure() -> ButtonBundle {
  ButtonBundle {
    style: Style {
      height: Val::Px(70.0),
      ..default()
    },
    background_color: BackgroundColor(Color::rgb(0.0, 0.0, 0.0)),
    ..default()
  }
}
