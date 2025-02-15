use bevy::prelude::*;
use bevy::app::AppExit;


pub mod screens_logic;
pub mod ui_elements;

pub mod consts {
  pub const LOGICAL_RESOULUTION: (f32, f32) = (640.0, 480.0);
  //pub const PHYSICAL_RESOULUTION: (f32, f32) = (1280.0, 720.0);
}


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum Screen {
    Splash,
    #[default]
    Title,
    Exit,
}


fn main() {
  let mut app = App::new();

  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        resolution:
          bevy::window::WindowResolution::new(
            consts::LOGICAL_RESOULUTION.0,
            consts::LOGICAL_RESOULUTION.1
          ),
        //mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..default()
      }),
      ..default()
    }))

    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

    .init_state::<Screen>()
    .add_systems(Startup, setup)
    .add_systems(Update, exit_check)

    .add_plugins((
      screens_logic::splash::plugin,
      screens_logic::title::plugin,
    ))

    .run();
}

pub fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn exit_check(
  state: Res<State<Screen>>,
  mut exit: EventWriter<AppExit>,
) {
  if *state.get() == Screen::Exit {
    exit.send(AppExit);
  }
}

// Generic system that takes a component as a parameter,
// and will despawn all entities with that component.
pub fn despawn_by_tag<T: Component>(
  to_despawn: Query<Entity, With<T>>,
  mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
