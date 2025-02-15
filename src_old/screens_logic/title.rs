use std::time::Duration;

use bevy::prelude::*;

use crate::Screen;
use crate::ui_elements;
use crate::despawn_by_tag;


#[derive(Component, Default)]
struct Tag;


pub fn plugin(app: &mut App) {
  app
    .add_systems(OnEnter(Screen::Title), (
      spawn_initial_entities,
    ).chain())

    //.add_systems(Update, (
    //  check_turn_on_visibility,
    //  check_turn_off_visibility,
    //  check_proceed_to_next_screen,
    //).run_if(
    //  in_state(Screen::Title)
    //))

    .add_systems(OnExit(Screen::Title), despawn_by_tag::<Tag>);
}


fn spawn_initial_entities(
  mut commands: Commands,
) {
  commands.spawn((
    Tag,
    ui_elements::root::get_element_structure(),
  )).with_children(|parent| {
    parent.spawn((
      ui_elements::title_container::get_element_structure(),
    )).with_children(|parent| {
      parent.spawn((
        ui_elements::title::get_element_structure(),
      ));
      parent.spawn((
        ui_elements::sub_title::get_element_structure(),
      ));
    });
    parent.spawn((
      ui_elements::menu_container::get_element_structure(),
    )).with_children(|parent| {
      parent.spawn((
        ui_elements::menu_button::get_element_structure(),
      )).with_children(|parent| {
        parent.spawn((
          ui_elements::menu_button_text::get_element_structure("Start"),
        ));
      });
      parent.spawn((
        ui_elements::menu_button::get_element_structure(),
      )).with_children(|parent| {
        parent.spawn((
          ui_elements::menu_button_text::get_element_structure("Settings"),
        ));
      });
      parent.spawn((
        ui_elements::menu_button::get_element_structure(),
      )).with_children(|parent| {
        parent.spawn((
          ui_elements::menu_button_text::get_element_structure("Quit"),
        ));
      });
    });
  });
}
