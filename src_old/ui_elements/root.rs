use bevy::prelude::*;

pub fn get_element_structure() -> NodeBundle {
  NodeBundle {
    style: Style {
      flex_direction: FlexDirection::Column,

      justify_content: JustifyContent::Center,
      align_content: AlignContent::Center,

      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      ..default()
    },
    ..default()
  }
}
