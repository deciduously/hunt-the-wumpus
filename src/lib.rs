#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

mod components;
mod game;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  game::Game,
  util::*,
};

use yew::prelude::*;

pub enum Model {
  GameOver(String),
  NewGame,
  Playing(Game),
}

impl Default for Model {
  fn default() -> Self {
    Model::NewGame
  }
}

#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  SwitchRoom(u8),
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model::default()
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    use self::Msg::*;

    match msg {
      SwitchRoom(target) => match self {
        Model::Playing(game) => {
          game.current_room = target;
          if let Some(msg) = game.move_effects() {
            *self = Model::GameOver(msg);
          };
        }
        _ => unreachable!(),
      },
      StartGame => *self = Model::Playing(Game::default()),
    }
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    use self::Model::*;

    match self {
      GameOver(s) => html! {
        <div class="hunt",>
          <span class="over-message",>{s}</span>
          <button onclick=|_| Msg::StartGame,>{"Play Again"}</button>
        </div>
      },
      Playing(game) => html! {
          <div class="hunt",>
              <div class="header",>{"Hunt the Wumpus"}</div>
              <div class="window",>
                <Stats: arrows=game.arrows, current_room=game.current_room,/>
                <Controls: exits=room_exits(game.current_room).unwrap(), onsignal=|msg| msg,/>
              </div>
              <Messages: messages=&game.messages,/>
          </div>
      },
      NewGame => html! {
        <div class="hunt",>
          <button onclick=|_| Msg::StartGame,>{"Start Game"}</button>
        </div>
      },
    }
  }
}
