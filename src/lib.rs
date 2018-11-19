#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  util::*,
};

use yew::prelude::*;

pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}

impl Model {
  fn configure_cave(&mut self) {
    self.messages.push(
      "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
    );
    self.wumpus = js_rand(1, 20);
    self.bats[0] = self.get_empty_room();
    self.bats[1] = self.get_empty_room();
    self.pits[0] = self.get_empty_room();
    self.pits[1] = self.get_empty_room();
  }
  
  fn get_empty_room(&self) -> u8 {
    gen_range_avoiding(0, 20, vec![self.current_room, self.wumpus, self.bats[0], self.bats[1], self.pits[0], self.pits[1]])
  }
}


#[derive(Debug, Clone)]
pub enum Msg {
  SwitchRoom(u8),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    let mut ret = Model {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
      wumpus: 0,
      bats: [0, 0],
      pits: [0, 0],
    };
    ret.configure_cave();
    ret
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::SwitchRoom(target) => {
        self.current_room = target;
        self.messages.push(format!("Moved to room {}", target));
        true
      }
    }
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="window",>
              <Stats: arrows=self.arrows, current_room=self.current_room,/>
              <Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
            </div>
            <Messages: messages=&self.messages,/>
        </div>
    }
  }
}
