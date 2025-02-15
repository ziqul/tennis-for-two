use belly::prelude::*;
use bevy::prelude::*;
use bevy::app::AppExit;


mod screens;

pub mod menus;
pub mod utils;


#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Splash,
    #[default]
    Title,
    Exit,
}


pub mod consts {
  pub const LOGICAL_RESOULUTION: (f32, f32) = (640.0, 480.0);
  //pub const PHYSICAL_RESOULUTION: (f32, f32) = (1280.0, 720.0);
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
    .add_plugins(BellyPlugin)

    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))

    .init_state::<GameState>()
    .add_systems(Startup, setup)
    .add_systems(Update, exit_check)

    .add_plugins((
        screens::splash::plugin,
        screens::title::plugin,
    ))
    .run();
}


pub fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}

fn exit_check(
  state: Res<State<GameState>>,
  mut exit: EventWriter<AppExit>,
) {
  if *state.get() == GameState::Exit {
    exit.send(AppExit);
  }
}
