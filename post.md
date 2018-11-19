# Hunt the Wumpus

In this post series we'll walk through recreating the classic [Hunt the Wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) game in [Yew](https://github.com/DenisKolodin/yew).  The original was played at the command line, we're going to use a webpage.

With Yew we will be defining our frontend in Rust, which will be compiled to [WebAssembly](https://webassembly.org/) for execution.

Does this app need this?  Does *any* app need this?

In order, no and debatable but probably.  Hash it out in the comments!

Will we do it anyway?  **HELL YES**.

Rust has some great tooling popping up making this compilation pipeline relatively painless.  Yew with `cargo-web` like I use here is only one of already several ways to go about it.  If you like what you find here I'd recommend the [RustWasm book](https://rustwasm.github.io/book/introduction.html) next.  It walks you through building a Game of Life `<canvas>` application without using any fancy frameworks or tools - from there you can pick and choose what you need on top of it.  You get to decide how low or high level you want to get with it.

This is a beginner-level tutorial - though it's helpful to be familiar with reading Rust.

## Setup

You'll need a nightly Rust compiler.  See [rustup](https://rustup.rs/) to get started if you need to - it's easy.  You'll also need [`cargo-web`](https://github.com/koute/cargo-web): `cargo install cargo-web`.  Once you have that installed navigate to your project directory and issue `cargo new hunt-the-wumpus` at the terminal.  Open that folder in the text editor of your choice.  We're going to start by adding the basic outline of the app and build pipeline.  Just enough to get everything compiling and running.

First we want to use the built-in Rust target.  Issue the following commands:

```
$ rustup override set nightly
$ echo 'default-target = "wasm32-unknown-unknown"' > Web.toml
```

This will ensure the `cargo web` command always uses the proper target.  The `override` command is directory-specific - to change it globally use `rustup default nightly`. I prefer to default to stable and only use nightly when necessary - like for the `wasm32-unknown-unknown` target.

Now make your `Cargo.toml` look like the following:

```toml
[package]
name = "hunt-the-wumpus"
version = "0.1.0"
authors = ["Hilbert Wumpfrey <hw@wumpusfood.gov>"]
edition = "2018"

[dependencies]
stdweb = "0.4"

[dependencies.yew]
git = "https://github.com/DenisKolodin/yew"
```

Most of our code is going to live in a library and the binary is just going to mount the app to the page.

Next replace your `main.rs` with the following:

```rust
extern crate hunt_the_wumpus;
extern crate yew;

use hunt_the_wumpus::Model;
use yew::prelude::App;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
```

This stub will just find our mount pount and attach our program to it.  Speaking of, let's create a mount point.  Issue:

```
$ mkdir static
$ touch static/index.html
```

We also just need a stub here.  Add the following to that file and save it:

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="utf-8">
  <meta http-equiv="X-UA-Compatible" content="IE=edge">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <meta name="description" content="Hunt the wumpus!">
  <meta name="author" content="YOU">
  <title>HUNT THE WUMPUS</title>
  <link rel="stylesheet" type="text/css" href="hunt.css">
  <script src="hunt.js"></script>
</head>

<body>
</body>

</html>
```

We won't need that again - it just loads `hunt.js`.  This `static` directory is where your favicon will go as well - I like [this one](https://www.favicon.cc/?action=icon&file_id=701981).

Now, let's add the basic Yew outline - the thing we're going to render.  Issue:

```
$ touch lib.rs
```

Fill it with the following template:

```rust
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
  arrows: i32,
}

#[derive(Debug, Clone)]
pub enum Msg {}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model { arrows: 5 }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="body",>
              <span class="arrows",>{&format!("Arrows: {}", self.arrows)}</span>
            </div>
        </div>
    }
  }
}
```

This is what most of our components are going to look like.  This should look somewhat familiar if you've used other frontend frameworks.  When the app is initialized we will `create` this component with the given `Model`, and Yew provides the `Renderable` trait and a JSX-like `html!` macro for defining the view.  It then draws inspiration from tools like Elm to provide a `Msg` type which will drive our events in the `update` method.  To start, `update` will always return `true`, triggering a redraw.

Before we get to coding, we need to set up the rest of the build pipeline.  We're going to use [`yarn`](https://yarnpkg.com/en/) - it's a web app, after all.

```
$ yarn init
// answer the questions
$ yarn add -D @babel/core @babel/preset-env autoprefixer node-sass nodemon npm-run-all postcss postcss-cli rollup rollup-plugin-babel rollup-plugin-postcss rollup-plugin-uglify rollup-plugin-wasm
```

Then add these scripts:

```json
  "scripts": {
    "clean": "cargo clean",
    "clean:deploy": "rm -rf release/",
    "build:rs": "cargo web deploy --release",
    "build:js": "rollup -c",
    "build:scss": "node-sass --include-path scss scss/hunt.scss css/hunt.css",
    "build:css": "postcss --use autoprefixer -o static/hunt.css css/hunt.css",
    "build:copy": "cp target/deploy/hunt.css release/ && cp target/deploy/hunt.wasm release/ && cp target/deploy/index.html release/ && cp target/deploy/favicon.ico release/",
    "build": "run-s clean:deploy build:rs build:js build:copy",
    "serve": "serve -p 8080 release/",
    "prod": "run-s build serve",
    "watch:rs": "cargo web start --release",
    "watch:scss": "nodemon -e scss -x \"yarn build:scss\"",
    "watch:css": "nodemon -e css -x \"yarn build:css\"",
    "watch": "run-p watch:rs watch:scss watch:css",
    "start": "run-s watch",
    "test": "echo \"Error: no tests!\" && exit 1"
  },
```

I'm going with SCSS, the choice is really up to you.  To follow along, issue:

```
$ mkdir scss
$ touch scss/hunt.scss
```

Just to make sure it's all hooked up, put the following in it:

```scss
.arrows {
  font-weight: bold;
}
```

Now, let's hit the big button:

```
$ yarn build:css-once
$ yarn watch:rs
```

Finally, point your browser to `localhost:8000`.  You should see the following:

**Hunt the Wumpus**
Arrows: 5

We're up and running!  Let's top off our `.gitignore`:

```
/target
**/*.rs.bk
/node_modules
yarn-*.log
/css
/static/*.css
```

Now, commit!  `git init && git commit -m "Initial commit`.

## **PART 2**

In the first part, we set up our development environment and ensured we can compile and run our webapp.  If you haven't done so, now's a good time to give it a look.

Now we can start modelling the logic.  First thing's first - let's define the cave.  The traditional game is played in a cave where each room is a vertex of a regular dodecahedron:

![dodecahedron](https://upload.wikimedia.org/wikipedia/commons/3/33/Dodecahedron.png)

From each room we are connected to exactly three other rooms.

To model this we'll simply use a function to map room IDs to available exits.  This will allow us to traverse around the cave.  Place the following in `lib.rs`, above your `Model` declaration:

```rust
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
    _ => None
  }
}
```

Now let's store the player's current location in the `Model`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
}
```

Don't forget to add it to our initial model too:

```rust
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
      arrows: 5,
      current_room: 1,
    }
  }
```

Now we can start adding to our UI.  We'll need a new component that will be responsible for rendering the controls.  I like keeping all of these in a folder:

```
$ mkdir src/components
$ touch src/components/controls.rs
```

We'll start with a barebones component:

```rust
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Controls {
    title: String,
    exits: [u8; 3],
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub exits: [u8; 3],
}

impl Default for Props {
    fn default() -> Self {
        Self { exits: [0, 0, 0] }
    }
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Controls {
            title: "Controls".into(),
            exits: props.exits,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>{format!("exits: {}, {}, {}", self.exits[0], self.exits[1], self.exits[2])}</div>
            </div>
        }
    }
}
```

Unlike our top-level component, this one accepts some props - we're going to pass in the exits to the room our player is in.  A couple of "gotchas" - take a look at the `html!` macro in the `Renderable` impl block.  We're attaching two classes to the top-level `div` - to do so, you need to wrap them up in a tuple like shown.  Also, if you're using an attribute in your tag like `<div class="title",>`, you need to include that trailing comma for the macro to work.  If you don't, you might end up with a very dense error message - check for these commas before panicking.  Rust macros tend to generate pretty opaque error info - one major drawback of the tech at this point in time.

Also of note - we *must* provide a `Default` impl for our `Props`.  I'm just setting it to `[0, 0, 0]`.

Let's position it within our larger app.  First, we have to organize our component module:

```
$ echo 'pub mod controls;' > src/components/mod.rs
```

When we add new components, don't forget to add the declaration to this file.  Back up in `lib.rs`, add the module directly after your `extern crate` declarations and bring it into scope:

```rust
mod components;

use components::controls::Controls;
```

Now we can attach it to the app.  Down in the `html!` macro, let's add the component right below our `<span>` element displaying the arrows.  We'll also section off the stats prinout:

```rust
<div class="body",>
  <div class=("container", "container-stats"),>
    <span class="title",>{"Stats"}</span>
    <span class="arrows",>{&format!("Arrows: {}", self.arrows)}</span>
    <br/>
    <span class="current-room",>{&format!("Current Room: {}", self.current_room)}</span>
  </div>
  <Controls: exits=room_exits(self.current_room).unwrap(),/>
</div>
```

Once the rebuild completes, go back to your browser and confirm you see:

**TODO** RE-RUN THIS AND SEE

**Hunt the Wumpus**
Stats
Arrows: 5
Current Room: 1
Controls
exits: 2, 5, 8

Gross, but just what we asked for!  Before we get too far into the logic, let's give ourselves something resembling a layout.  This is just going to be a skeleton - I'm no CSS guru.  Feel free to make this whatever you like, this should be enough to get you started.

Replace `scss/hunt.scss` with the following:

```scss
.hunt {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  width: 100%;

  .header {
    flex: 0;
    font-size: 36px;
    font-weight: bold;
    text-align: center;
  }
  
  .window {
    display: flex;
    flex-direction: row;
  }

  .container {
      border: solid 1px #000;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      margin: 10px;
      padding: 5px;
  
      >.title {
          border-bottom: dashed 1px #000;
          font-weight: bold;
          text-align: center;
      }
    
      >.scroller {
          overflow: auto;
      }
  }

  .container-stats {
    flex: 0 0 256px;
    order: 0;
  
    .stat {
      font-style: italic;
    }
  }

  .container-control {
    flex: 0 0 256px;
    order: 1;

    .control-button {
      flex: 1;
    }
  }
}
```

Let's also go ahead and take the opportunity to just break out the Stats out into their own component.  Make a new file `src/components/stats.rs`:

```rust
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Stats {
  title: String,
  arrows: u8,
  current_room: u8,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
  pub arrows: u8,
  pub current_room: u8,
}

impl Default for Props {
  fn default() -> Self {
    Self {
      arrows: 0,
      current_room: 0,
    }
  }
}

impl Component for Stats {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Stats {
      title: "Stats".into(),
      arrows: props.arrows,
      current_room: props.current_room,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Stats> for Stats {
  fn view(&self) -> Html<Self> {
    html! {
      <div class=("container", "container-stats"),>
        <span class="title",>{&self.title}</span>
        <span class="stat",>{&format!("Arrows: {}", self.arrows)}</span>
        <br/>
        <span class="stat",>{&format!("Current Room: {}", self.current_room)}</span>
      </div>
    }
  }
}
```

New we just add it to `src/components/mod.rs`:

```rust
pub mod controls;
pub mod stats;
```

and include it in our top level component in `lib.rs`:

```rust
mod components

use self::components::{controls::Controls, stats::Stats};

// down to the bottom...

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="window",>
              <Stats: arrows=self.arrows, current_room=self.current_room,/>
              <Controls: exits=room_exits(self.current_room).unwrap(),/>
            </div>
        </div>
    }
  }
}
```

This gives us a simple flexbox layout that will be easy to extend.  Re-run `yarn build:css-once` and reload `localhost:8000` in your browser.

**SCREENSHOT**

Aww yiss.  Check out Part 3 to get **interactive** with it.

## Part 3 - THE BUTTONS

Our next order of business is moving around the cave.  All of our actual update logic is going to happen in our top-level component.  When we first created `lib.rs`, we just made an empty `Msg` type:

```rust
#[derive(Debug, Clone)]
pub enum Msg {}
```

To switch `current_room`, we're going to send a `Msg` containing the target room. Let's add the variant first:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  SwitchRoom(u8),
}
```

Now we have to handle that message.  Inside the `impl Component for Model` block we currently have a stub for `update()`, returning `true`.  Now lets actually use the `Self::Message` parameter it accepts:

```rust
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::SwitchRoom(target) => {
        self.current_room = target;
        true
      }
    }
  }
```

The great thing about using an `enum` for your messages is that the compiler won't let you miss any when you `match` on them - it must be exhaustive.  We also get to easily destructure the variant.  This pattern is not unlike what Elm offers.  You just need to make sure each match arm returns a boolean - or if you like, you can simply return `true` after the `match` block.  Controlling on a per-message basis may allow for more granular performance control - some messages may not require a re-render.

This message is simple - it just switches `current_room`.  Next we need to generate these messages.  Let's dive back in to `src/components/controls.rs`.  We can use `crate::Msg` to refer to the toplevel message our buttons will generate.

We can now create a message that can be passed within this component:

```rust
pub enum Msg {
    ButtonPressed(crate::Msg)
}
```

We also need to add the callback to our props.  Yew has a type ready to go, add it to your imports:

```rust
use yew::prelude::{Callback, Component, ComponentLink, Html, Renderable, ShouldRender};
```

Now we can use it in our `Props` and component struct:

```rust
pub struct Controls {
    title: String,
    exits: [u8; 3],
    onsignal: Option<Callback<crate::Msg>>,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub exits: [u8; 3],
    pub onsignal: Option<Callback<crate::Msg>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            exits: [0, 0, 0],
            onsignal: None,
        }
    }
}
```

Finally, add it to our component initalization:

```rust
fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Controls {
        title: "Controls".into(),
        exits: props.exits,
        onsignal: props.onsignal,
    }
}
```

Now we can dynamically create buttons to generate our `HuntMsg`.  We already have the room targets coming in to the component - we just need a way to create a different button for each.  We can abstract this logic out with a local closure in our `view` function:

```rust
impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        let view_button = |target: &u8| {
            use crate::Msg::*;
            let t = *target;
            html! {
                <span class="control-button",>
                    <button onclick=|_| Msg::ButtonPressed(SwitchRoom(t)),>{&format!("Move to {}", target)}</button>
                </span>
            }
        };
        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>{ for self.exits.iter().map(view_button) }</div>
            </div>
        }
    }
}
```

We then map `view_button` over the exits in our state.  Another gotcha - you've got to dereference `target` outside of the `html!` macro: `let t = *target`.  If our type wasn't `Copy` like `u8`, we'd need to clone it here.

Now we need to handle the message.  Let's fill in our `update`:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
        Msg::ButtonPressed(msg) => {
            if let Some(ref mut callback) = self.onsignal {
                callback.emit(msg);
            }
        }
    }
    false
}
```

No need to re-render on the click - we'll handle that later when the state actually changes.  We return `false` to make sure we dont waste time on an exra render.  Now we just add the prop to `lib.rs`, down in the `view` functiom:

```rust
 <Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
```

When the button is clicked the `msg` will fire and our toplevel `update` will handle changing the state.  Now we can pass any message we want up as a callback.

There's one final change to make before it all works - we need to tell any component that takes `Props` what do do when those props change.  Define these  `change` functions in the `impl Component for <...>` blocks.

First, `controls.rs`:

```rust
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.exits = props.exits;
        self.onsignal = props.onsignal;
        true
    }
```

Then `stats.rs`:

```rust
  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.arrows = props.arrows;
    self.current_room = props.current_room;
    true
  }
```

Now make sure your `yarn watch:rs` watcher is running and open up `localhost:8000`.  You should be able to use the buttons to "explore" the maze.  To keep track of where we've been, let's display a running history for the player.

First, we'll add a field to our toplevel state in `lib.rs`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
}

impl Component for Model {
   // ..
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
    }
  }
  // ..
}
```

We'll add a new component in a new file `src/components/messages.rs`:

```rust
use yew::prelude::{Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Messages {
  title: String,
  messages: Vec<String>,
}

pub enum Msg {}

#[derive(PartialEq, Clone)]
pub struct Props {
  pub messages: Vec<String>,
}

impl Default for Props {
  fn default() -> Self {
    Props {
      messages: Vec::new(),
    }
  }
}

impl Component for Messages {
  type Message = Msg;
  type Properties = Props;

  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Messages {
      title: "Messages".into(),
      messages: props.messages,
    }
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }

  fn change(&mut self, props: Self::Properties) -> ShouldRender {
    self.messages = props.messages;
    true
  }
}

impl Renderable<Messages> for Messages {
  fn view(&self) -> Html<Self> {
    let view_message = |message: &String| {
      html! {
          <li>{message}</li>
      }
    };
    html! {
        <div class=("container", "container-messages"),>
            <div class="title",>{&self.title}</div>
            <div class="scroller",>
                <ul>{ for self.messages.iter().rev().map(view_message) }</ul>
            </div>
        </div>
    }
  }
}
```

We're showing the messages in reverse - otherwise, this isn't too different from `controls.rs`.  Protip - I use a snippet something like this when I'm starting a new component!

Don't forget to add it to `src/components/mod.rs`:

```rust
pub mod controls;
pub mod messages;
pub mod stats;
```

And add it to `lib.rs`:

```rust
use self::components::{controls::Controls, messages::Messages, stats::Stats};

// ..

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
        <div class="hunt",>
            <div class="header",>{"Hunt the Wumpus"}</div>
            <div class="window",>
              <Stats: arrows=self.arrows, current_room=self.current_room,/>
              <Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
            </div>
            <Messages: messages=&self.messages,/> // add it down here
        </div>
    }
  }
}
```

Now let's add a little style in `scss/hunt.scss`.  Add the followng below the `>.title` block inside the `.container` block:

```scss
      >.scroller {
          overflow: auto;
      }
```

and then add right at the end:

```scss
.hunt {
// ..
  .container-messages {
    flex: 0 0 192px;
    
    ul {
      list-style-type: none;
    }
  }
}
```

Now let's add some!  First, we need an action for it:

```rust

```

Let's welcome the player to their likely doom when the game initiates in `lib.rs`:

```rust
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
```

We'll also log each move:

```rust
  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::SwitchRoom(target) => {
        self.current_room = target;
        self.messages.push(format!("Moved to room {}", target));
        true
      }
    }
  }
```

**SCREENSHOT**

Nifty!  Join me in Part 4 for some peril.

**PART 4** - THE DANGER

Our cave isn't terribly interesting.  There's some low-hanging fruit, here - there's gotta be a wumpus to hunt!

Open up `src/lib.rs` and add one to our `Model`:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
}
```

We need a placeholder starting position - there is no room 0, our cave rooms are 1-indexed:

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
  let mut rng = thread_rng();
  let mut ret = Model {
    arrows: 5,
    current_room: 1,
    messages: Vec::new(),
    wumpus: 0,
  };
  // ..
}
```

We'll place him in a moment.  That's not quite scary enough, though.  In addition to the ravenous monstrosity loafing about there are two gigantic bats.  If you end up in a room with a bat, it'll quite literally sweep you off your feet - 

Now, we're gonna crank the horror up to eleven.  Forget the two chaos-inducing hellbats.  There are also two rooms that are bottomless pits.  What the flip, man. **Bottomless**.  You'll die of thirst, after three days of falling.  Gives me the crimineys, I'll tell you hwat.

We'll initialize the variables to 0 and place them later, to amke sure we scatter them about:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}
let mut ret = Model {
  arrows: 5,
  current_room: 1,
  messages: Vec::new(),
  wumpus: 0,
  bats: [0, 0],
  pits: [0, 0],
};
```

To place the horribleness, we'll use a helper function that will generate random numbers avoiding a list that we specify.

We're going to call out out to JS to generate the random number.  First add the `#[macro_use]` annotation to the `extern crate stdweb` line in `lib.rs`.  Now, this is going to be our second helper function and I don't want to clutter up `lib.rs` too much, so lets create a file called `src/util.rs`:

```rust
use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
  let rand = js! { return Math.random(); };
  let base: f64 = rand.try_into().unwrap();
  (base * top as f64).floor() as u8 + bottom
}

pub fn gen_range_in_range_avoiding(bottom: u8, top: u8, avoid: Vec<u8>) -> u8 {
  let mut ret = avoid[0];
  while avoid.contains(&ret) {
    ret = js_rand(bottom, top);
  }
  ret
}
```

The `js_rand` function wraps up our interop so we deal with Rust types as much as we can - we only need JS for the entropy. The helper `gen_range_avoiding` will give us back a `u8` that doesn't appear in `avoid`.



We can also move `room_exits` from `lib.rs` into this file and mark it `pub`.  Don't forget to add it to the top of `lib.rs`:

```rust
mod components;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  util::*,
};
```

To make this utility easier to use, let's give `Model` a method for it in `lib.rs`, along with a `configure_cave()` method to place all of our sadistic traps:

```rust
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
```

With all this danger lurking around every corner, we should give the player a few warnings as they're stepping around.

Let's add another method to `Model` to sniff around our surroundings:

```rust
```