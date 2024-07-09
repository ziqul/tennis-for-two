use belly::prelude::*;
use belly::core::eml::Eml;
use bevy::prelude::*;


#[derive(Component)]
struct Counter {
    count: i32,
}


pub fn get_nodes(
  commands: &mut Commands,
) -> Eml {
  let counter = commands.spawn(Counter {
    count: 2,
  }).id();

  eml! {
    <label
      s:font-size="50px"
      s:color="#ffffff"
      bind:value=from!(counter, Counter:count|fmt.c("Value: {c}"))
    />
  }
}
