extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;

use self::components::{controls::Controls, messages::Messages, stats::Stats};
use yew::prelude::*;

fn room_exits(id: u8) -> Option<[u8; 3]> {
  match id {
    1 => Some([2, 5, 8]),
    2 => Some([1, 3, 10]),
    3 => Some([2, 4, 12]),
    4 => Some([3, 5, 14]),
    5 => Some([1, 4, 6]),
    6 => Some([5, 7, 15]),
    7 => Some([6, 8, 17]),
    8 => Some([1, 7, 11]),
    9 => Some([10, 12, 19]),
    10 => Some([2, 9, 11]),
    11 => Some([8, 10, 20]),
    12 => Some([3, 9, 13]),
    13 => Some([12, 14, 18]),
    14 => Some([4, 13, 15]),
    15 => Some([6, 14, 16]),
    16 => Some([15, 17, 18]),
    17 => Some([7, 16, 20]),
    18 => Some([13, 16, 19]),
    19 => Some([9, 18, 20]),
    20 => Some([11, 17, 19]),
    _ => None,
  }
}

pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
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
    };
    ret.messages.push(
      "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
    );
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
