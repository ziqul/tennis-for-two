use bevy::prelude::*;

pub fn get_element_structure() -> NodeBundle {
  NodeBundle {
    style: Style {
      flex_direction: FlexDirection::Column,

      justify_content: JustifyContent::Center,
      align_content: AlignContent::Center,

      justify_items: JustifyItems::Center,
      align_items: AlignItems::Center,
      ..default()
    },
    ..default()
  }
}
