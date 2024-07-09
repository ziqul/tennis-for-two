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


fn main() {
  let mut app = App::new();

  app
    .add_plugins(DefaultPlugins)
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
