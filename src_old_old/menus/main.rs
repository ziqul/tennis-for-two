use belly::prelude::*;
use belly::core::eml::Eml;
use bevy::prelude::*;


#[derive(Component, Default)]
struct Selectable {
  selected: bool,
}

pub fn get_nodes(
  commands: &mut Commands,
) -> Eml {
  eml! {
    <div
      s:flex-direction="column"
    >

      <div
        with=({ Selectable { selected: true } })
        s:flex-direction="row"
      >
        <label
          id="selection-marker1"
          s:font-size="50px"
          s:color="#ffffff"
          value="> "
        />
        <label
          s:font-size="50px"
          s:color="#ffffff"
          value="Start"
        />
      </div>

      <div
        with=Selectable
        s:flex-direction="row"
      >
        <label
          id="selection-marker"
          s:font-size="50px"
          s:color="#ffffff"
          value="> "
        />
        <label
          s:font-size="50px"
          s:color="#ffffff"
          value="Settings"
        />
      </div>

      <div
        with=Selectable
        s:flex-direction="row"
      >
        <label
          id="selection-marker"
          s:font-size="50px"
          s:color="#ffffff"
          value="> "
        />
        <label
          s:font-size="50px"
          s:color="#ffffff"
          value="Quit"
        />
      </div>

    </div>
  }
}

//pub fn init_selectable(
//  mut elements: Elements,
//  mut query: Query<(&mut Element, &mut Selectable)>,
//) {
//  for (mut element, mut selectable) in &mut query {
//    element.children;
//  }
//}
