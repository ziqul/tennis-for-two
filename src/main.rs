use bevy::prelude::*;


mod splash;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    //Menu,
    //Game,
    Exit,
}


fn main() -> AppExit {
  let mut app = App::new();

  app
    .add_plugins(DefaultPlugins)
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))

    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Update, exit_check)

    .add_plugins((
      splash::plugin,
    ))
    .run()
}


pub fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter,
// and will despawn all entities with that component.
fn despawn_by_tag<T: Component>(
  to_despawn: Query<Entity, With<T>>,
  mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn create_ui_root() -> NodeBundle {
  NodeBundle {
    style: Style {
      flex_direction: FlexDirection::Column,
      align_items: AlignItems::Center,
      justify_content: JustifyContent::Center,
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      ..default()
    },
    ..default()
  }
}

fn exit_check(
  state: Res<State<GameState>>,
  mut exit: EventWriter<AppExit>,
) {
  if *state.get() == GameState::Exit {
    exit.send(AppExit::Success);
  }
}
