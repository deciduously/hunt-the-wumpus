# Wumpus Season

In this post series we'll walk through recreating the classic [Hunt the Wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) game in [Yew](https://github.com/DenisKolodin/yew).  The original was played at the command line, we're going to use a webpage.  Yew allows us to define our frontend in Rust.  Our app will be compiled to [WebAssembly](https://webassembly.org/) for execution.

Does this app need this?  *No.*

Does *any* app need this?  Debatable, but probably.  Hash it out in the comments!

Will we do it anyway?  **HELL YES!**

This is a beginner-level tutorial - it's helpful to be familiar with reading Rust but there's nothing too fancy going on here.  Comfort in any imperative language should be sufficient.

I've split this into three parts and will post them throughout this week.  This first part is designed to stand alone as a useful guide for starting your own blank project.  No wumpus hunting yet, just replace the filler text with stuff appropriate for your app.  Part 2 sets up our basic UI and mechanism for moving around the cave and Part 3 discusses the game logic.

## Setup

Rust has some great tooling popping up making this compilation pipeline relatively painless.  Yew with `cargo-web` like we use is only one of already several ways to go about it.  If you like what you find here I'd recommend the [RustWasm book](https://rustwasm.github.io/book/introduction.html) next.  It walks you through building a Game of Life `<canvas>` application without using any fancy frameworks or tools - from there you can pick and choose what you need on top of it.  You get to decide how low or high level you want to get with it.  Also be sure to check out [draco](https://github.com/utkarshkukreti/draco), an alternative client-side Rust->Wasm framework.

You'll need a nightly Rust compiler.  See [rustup](https://rustup.rs/) to get started if you need to - it's easy.  You'll also need [`cargo-web`](https://github.com/koute/cargo-web): `cargo install cargo-web`.

Once you have that installed navigate to your projects directory and issue `cargo new hunt-the-wumpus` at the terminal.  Open that folder in the text editor of your choice.  We're going to start by adding just enough to get everything compiling and running.

First lets set up our project folder to use the built-in Rust target.  Issue the following commands:

```
$ rustup override set nightly
$ echo 'default-target = "wasm32-unknown-unknown"' > Web.toml
```

This will ensure the `cargo web` command always uses the proper target.  The `rustup override` command is directory-specific - to change it globally use `rustup default nightly`. I prefer to default to stable and only use nightly when necessary.

Now make your `Cargo.toml` look like the following:

```toml
[package]
authors = ["Hunter Wumpfrey <hw@bottomlesspit.net>"]
edition = "2018"
name = "hunt-the-wumpus"
version = "0.1.0"
[[bin]]
name = "hunt"
path = "src/main.rs"

[dependencies]
stdweb = "0.4"

[dependencies.yew]
git = "https://github.com/DenisKolodin/yew"

[lib]
name = "hunt_the_wumpus"
path = "src/lib.rs"
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

We won't need that again - it just loads up our compiled JS and our stylesheet.  This `static` directory is where your favicon will go as well - I like [this one](https://www.favicon.cc/?action=icon&file_id=701981).

Now, let's add the basic Yew outline - the thing we're going to render.  Issue:

```
$ touch src/lib.rs
```

Fill it with the following template:

```rust
extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {
  arrows: u8,
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

This is what most of our components are going to look like.  This should look somewhat familiar if you've used other frontend frameworks.  There's a `Component` trait where we can define state transformations like `create` and `update` and a `Renderable<T>` trait with a JSX-like `html!` macro for defining the view.  It then draws inspiration from tools like Elm to provide a `Msg` type which will drive our events in the `update` method.  We don't have any messages to process yet, so we're just including a stub.  To start off `update` will always return `true` for `ShouldRender`, triggering a redraw.

Before we get to coding, we need to set up the rest of the build pipeline.  We're going to use [`yarn`](https://yarnpkg.com/en/) - it's a web app, after all.

```
$ yarn init
// answer the questions
$ yarn add -D @babel/core @babel/preset-env autoprefixer node-sass nodemon npm-run-all postcss postcss-cli rollup rollup-plugin-babel rollup-plugin-postcss rollup-plugin-uglify rollup-plugin-wasm serve
```

Then add these scripts to your `package.json`:

```json
  "scripts": {
    "build:js": "rollup -c",
    "build:rs": "cargo web deploy --release",
    "build:scss": "node-sass --include-path scss scss/hunt.scss css/hunt.css",
    "build:css": "postcss --use autoprefixer -o static/hunt.css css/hunt.css",
    "build:style": "run-s build:scss build:css",
    "build:copy": "cp target/deploy/hunt.css release/ && cp target/deploy/hunt.wasm release/ && cp target/deploy/index.html release/ && cp target/deploy/favicon.ico release/",
    "build": "run-s clean:deploy build:rs build:js build:style build:copy",
    "clean:deploy": "rm -rf /release",
    "prod": "run-s build serve",
    "serve": "serve -p 8080 release",
    "watch:rs": "cargo web start --release",
    "test": "echo \"Error: no tests!\" && exit 1"
  },
```

To set up our app-wide stylesheet, issue:

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

Now, let's hit the big button.  Open your terminal and issue

```
$ yarn build:style
$ yarn watch:rs
```

Finally, point your browser to `localhost:8000`.  You should see the following:

Hunt the Wumpus
**Arrows: 5**

We're up and running!  The development config works.  Let's top off our `.gitignore`:

```
/target
**/*.rs.bk
/node_modules
yarn-*.log
/css
/static/*.css
/release
```

Let's test our our production bundle.  First create `rollup.config.js` and save the following contents:

```js
import babel from "rollup-plugin-babel"
import uglify from "rollup-plugin-uglify"

export default {
    input: './target/deploy/hunt.js',
    output: {
        name: 'hunt',
        file: './release/hunt.js',
        format: 'es',
    },
    plugins: [
        babel({
            exclude: 'node_modules/**'
        }),
        uglify
    ]
};
```

Now make sure you exit the `watch:rs` process, and then try `yarn prod`.  When the build completes, you should see the same output at `localhost:8080`.

Once it's all working, commit!  `git init && git commit -m "Initial commit`.  Tomorrow we'll dive in to the build.

See [here](https://github.com/deciduously/hunt-the-wumpus/tree/master/part1) for the full code at the end of part 1.

**PART 2**
*********************************************************************************************************************************************************************************************************************************************************************************
*********************************************************************************************************************************************************************************************************************************************************************************

In the first part, we set up our development environment and ensured we can compile and run our webapp.  If you haven't done so, now's a good time to give it a look.  This part starts assuming your project folder mirrors [this one](https://github.com/deciduously/hunt-the-wumpus/tree/master/part1)

Now we can start modelling the logic.  we'll start by defining the cave.  The traditional game is played in a cave where each room is a vertex of a regular dodecahedron:

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
use yew::prelude::*;

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

Let's position it within our app.  First, we have to organize our component module:

```
$ echo 'pub mod controls;' > src/components/mod.rs
```

When we add new components, don't forget to add the declaration to this file.  Back up in `lib.rs`, add the module directly after your `extern crate` declarations and bring it into scope:

```rust
mod components;

use self::components::controls::Controls;
```

Now we can attach it to the app.  Down in the `html!` macro let's add the component right below our `<span>` element displaying the arrows.  We'll also section off the stats printout and display the current room.  Adjust yours to match this:

```rust
<div class="hunt",>
    <div class="header",>{"Hunt the Wumpus"}</div>
    <div class="body",>
      <div class=("container""container-stats"),>
        <span class="title",>{"Stats"}</span>
        <br/>
        <span class="arrows",>{&forma("Arrows: {}", self.arrows)}</span>
        <br/>
        <span class="current-room",>{&forma("Current Room: {}"self.current_room)}</span>
      </div>
      <Controls: exits=room_exi(self.current_room).unwrap(),/>
    </div>
</div>
```

Once the rebuild completes, go back to your browser and confirm you see:

Stats
**Arrows: 5**
Current Room: 1
Controls
exits: 2, 5, 8

Pretty plain, but just what we asked for!  Before we get too far into the logic, let's give ourselves something resembling a layout.  This is just going to be a skeleton - I'm no CSS guru.  Feel free to make this whatever you like, this should be enough to get you started.

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
  }
}
```

Don't forget to run `yarn build:style` to regenerate the compiled CSS.

Let's also go ahead and take the opportunity to just break out the Stats out into their own component.  Make a new file `src/components/stats.rs`:

```rust
use yew::prelude::*;

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
mod components;

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

Aww yiss.  Now we're ready to get **interactive** with it.

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

Don't forget to remove the underscore from `_msg` in the parameter list!

The great thing about using an `enum` for your messages is that the compiler won't let you miss any when you `match` on them - it must be exhaustive.  We also get to easily destructure the variant.  This pattern is not unlike what Elm offers.  You just need to make sure each match arm returns a boolean - or if you like, you can simply return `true` after the `match` block.  Controlling on a per-message basis may allow for more granular performance control - some messages may not require a re-render.

This message is simple - it just switches `current_room`.  Next we need to generate these messages.  Let's dive back in to `src/components/controls.rs`.  We can use `crate::Msg` to refer to the toplevel message our buttons will generate.

We can now create a message that can be passed within this component:

```rust
pub enum Msg {
    ButtonPressed(crate::Msg)
}
```

We also need to add the callback to our props.  Yew has a type ready to go:

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

Now we can dynamically create buttons to generate our `crate::Msg`.  We already have the room targets coming in to the component - we just need a way to create a different button for each.  We can abstract this logic out with a local closure in our `view` function:

```rust
impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        let move_button = |target: &u8| {
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
                <div class="exits",>{ for self.exits.iter().map(move_button) }</div>
            </div>
        }
    }
}
```

We then map `move_button` over the exits in our state.  Another gotcha - you've got to dereference `target` outside of the `html!` macro: `let t = *target`.  If our type wasn't `Copy` like `u8`, we'd need to clone it here.

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

No need to re-render on the click.  We'll handle that later when the state actually changes.  We return `false` to make sure we dont waste time on an exra render.  Now we just add the prop to `lib.rs`, down in the `view` functiom:

```rust
<Controls: exits=room_exits(self.current_room).unwrap(), onsignal=|msg| msg,/>
```

When the button is clicked the `msg` will fire and our toplevel `update` will handle changing the state.  Now we can pass any message we want up as a callback.

There's one final change to make before it all works - we need to tell any component that takes `Props` what do do when those props change.  Define these  `change` functions in the `impl Component for <...>` blocks of these respective components:

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

Now make sure your `yarn watch:rs` watcher is running and open up `localhost:8000`.  You should be able to use the buttons to "explore" the maze.

To keep track of where we've been, let's display a running history for the player.  First, we'll add a field to our toplevel state in `lib.rs`:

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
use yew::prelude::*;

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

Now let's add a little style in `scss/hunt.scss`.  Add the following below the `>.title` block inside the `.container` block:

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

To pull in the changes, run `yarn build:style`.

Now let's add some messages!  We can welcome the player to their likely doom when the game initiates in `lib.rs`:

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

Nifty!  Our cave isn't terribly interesting though.  There's some low-hanging fruit, here - there's gotta be a wumpus to hunt!

Join me in Part 3 to make a game out of this treacherous dodecacave.

**PART 3**
*********************************************************************************************************************************************************************************************************************************************************************************
*********************************************************************************************************************************************************************************************************************************************************************************

This is the third and final part of a 3 part series.  Thhis post starts off asuming you've got a project that looks something like [this](https://github.com/deciduously/hunt-the-wumpus/tree/master/part2).  Here are links for [Part 1](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-1-3k2o) and [Part 2](https://dev.to/deciduously/lets-build-a-rust-frontend-with-yew---part-2-1ech) if you need to catch up.

Part 2 left us with a cave we can wander around, but not much in the way of danger.  The name of the game is "Hunt the Wumpus" and there's nary a wumpus in sight!

Open up `src/lib.rs`.  Let's add one to our `Model`:

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

We'll place him in a moment.  That's not quite scary enough, though.  In addition to the ravenous monstrosity loafing about there are two gigantic bats.  If you end up in a room with a bat, it'll quite literally sweep you off your feet and deposit you elsewhere in the cave.

Now we're gonna crank the horror up to eleven.  Forget the two chaos-inducing hellbats.  There are also two rooms that are bottomless pits.  What the flip, man. **Bottomless**.  You'll die of thirst, after three days of falling.  Gives me the crimineys, I'll tell you hwat.

We'll initialize the variables to 0 and place them later:

```rust
pub struct Model {
  arrows: u8,
  current_room: u8,
  messages: Vec<String>,
  wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}
```

Let's go ahead and implement `Default` for `Model` with some zeros for everything that we can configure later:

```rust
impl Default for Model {
  fn default() -> Self {
    Self {
      arrows: 5,
      current_room: 1,
      messages: Vec::new(),
      wumpus: 0,
      bats: [0, 0],
      pits: [0, 0],
    }
  }
}
```

To place the horribleness, we'll use a helper function that will generate random numbers avoiding a list that we specify.

We're going to call out out to JS to generate the random number.  First add the `#[macro_use]` annotation to the `extern crate stdweb` line in `lib.rs`:

```rust
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
```

I don't want to clutter up `lib.rs` too much, so lets create a file called `src/util.rs`:

```rust
use stdweb::unstable::TryInto;

pub fn js_rand(bottom: u8, top: u8) -> u8 {
  let rand = js! { return Math.random(); };
  let base: f64 = rand.try_into().unwrap();
  (base * top as f64).floor() as u8 + bottom
}

pub fn gen_range_avoiding(bottom: u8, top: u8, avoid: Vec<u8>) -> u8 {
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

To make this utility easier to use, let's give `Model` a method for it in `lib.rs`, along with a `configure_cave()` method to initiate our world and place all of our sadistic traps:

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
    self.warning_messages();
  }

  fn get_empty_room(&self) -> u8 {
    gen_range_avoiding(
      0,
      20,
      vec![
        self.current_room,
        self.wumpus,
        self.bats[0],
        self.bats[1],
        self.pits[0],
        self.pits[1],
      ],
    )
  }
}
```

Now we can rewrite our `create` function:

```rust
fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
  let mut ret = Model::default();
  ret.configure_cave();
  ret
}
```

With all this danger lurking around every corner, we should give the player a few warnings as they're stepping around.

Let's add another method to `Model` to sniff around our surroundings.  If any of our adjacent rooms has a hazard, we'll alert the player with a spooky message.  Add this to the `impl Model` block:

```rust
fn warning_messages(&mut self) {
  for adj in &room_exits(self.current_room).unwrap() {
    let t = *adj;
    if self.wumpus == t {
      self
        .messages
        .push("You smell something horrific and rancid.".into());
    } else if self.pits.contains(&t) {
      self
        .messages
        .push("You feel a cold updraft from a nearby cavern.".into());
    } else if self.bats.contains(&t) {
      self
        .messages
        .push("You hear a faint but distinct flapping of wings.".into());
    }
  }
}
```

We can check for nearby hazards whenever we move:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
  match msg {
    Msg::SwitchRoom(target) => {
      self.current_room = target;
      self.messages.push(format!("Moved to room {}", target));
      self.warning_messages();
      true
    }
  }
}
```

Before we start dealing with larger level states, let's go ahead and abstract out our `Game` from our `Model`.  Create a new file called `src/game.rs`.  We're going to pull a lot of the logic we had defined on `Model` and put it here instead.

```rust
use crate::util::*;

pub struct Game {
  pub arrows: u8,
  pub current_room: u8,
  pub messages: Vec<String>,
  pub wumpus: u8,
  bats: [u8; 2],
  pits: [u8; 2],
}

impl Game {
  fn configure_cave(&mut self) {
    self.messages.push(
      "You've entered a clammy, dark cave, armed with 5 arrows.  You are very cold.".to_string(),
    );
    self.wumpus = js_rand(1, 20);
    self.bats[0] = self.get_empty_room();
    self.bats[1] = self.get_empty_room();
    self.pits[0] = self.get_empty_room();
    self.pits[1] = self.get_empty_room();
    self.warning_messages();
  }

  fn get_empty_room(&self) -> u8 {
    gen_range_avoiding(
      0,
      20,
      vec![
        self.current_room,
        self.wumpus,
        self.bats[0],
        self.bats[1],
        self.pits[0],
        self.pits[1],
      ],
    )
  }

  pub fn warning_messages(&mut self) {
    for adj in &room_exits(self.current_room).unwrap() {
      let t = *adj;
      if self.wumpus == t {
        self
          .messages
          .push("You smell something horrific and rancid.".into());
      } else if self.pits.contains(&t) {
        self
          .messages
          .push("You feel a cold updraft from a nearby cavern.".into());
      } else if self.bats.contains(&t) {
        self
          .messages
          .push("You hear a faint but distinct flapping of wings.".into());
      }
    }
  }
}

impl Default for Game {
  fn default() -> Self {
    let mut ret = Self {
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
}
```

Bring everything into scope in `lib.rs`:

```rust
mod components;
mod game;
mod util;

use self::{
  components::{controls::Controls, messages::Messages, stats::Stats},
  game::Game,
  util::*,
};
```

We also moved the "new game" setup into the `Default` implementation. We're going to have to make some changes to `lib.rs`.  First, we're going to define a few different types of `Model` we want to be able to render.  Change your `struct` to this `enum`:

```rust
pub enum Model {
  Waiting(String),
  Playing(Game),
}
```

Now we have a gamestate for when there isn't an active game.  You can remove the old `impl Model` block - that logic ll ended up in `game.rs`.  When the app starts, we want to open the `NewGame` state:

```rust
impl Default for Model {
  fn default() -> Self {
    Model::Waiting("New Game!".into())
  }
}

impl Component for Model {
  // ..
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model::default()
  }
  // ..
```

We need a message to kick off a new game:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  SwitchRoom(u8),
}
```

This will require a few changes to our `update` function too.  We have a new message to handle, and we need to do some extra checking to make sure we're in a gamestate that makes sense:

```rust
fn update(&mut self, msg: Self::Message) -> ShouldRender {
  use self::Msg::*;
  match msg {
    SwitchRoom(target) => match self {
      Model::Playing(game) => {
        game.current_room = target;
        game.warning_messages();
      }
      _ => unreachable!(),
    },
    StartGame => *self = Model::Playing(Game::default()),
  }
  true
}
```

We've now got to make sure we're playing a game before switching rooms but we can send the `StartGame` message to reroll the gamestate at any time.

Finally, we add a match arm for each game state in our `view`:

```rust
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
  }
}
```

Each state has it's own `html!` macro to render.  For good measure, add a little style just below the final closing brace in `hunt.scss`:

```rust
.over-message {
  font-size: 22px;
  color: red;
}
```

Over in `game.rs` lets flesh out everything that we want to check on a move end.  Add a new method in our `impl Game` block:

```rust
pub fn move_effects(&mut self) -> Option<String> {
  self.warning_messages();
  if self.current_room == self.wumpus {
    Some("You have been eaten slowly and painfully by the wumpus".into())
  } else if self.pits.contains(&self.current_room) {
    Some(
      "You have fallen into a bottomless pit and must now wait to die, falling all the while"
        .into(),
    )
  } else if self.bats.contains(&self.current_room) {
    // Switch us to a random room
    let current = self.current_room;
    let next = self.get_empty_room();
    self.messages.push(format!(
      "A gigantic bat whisks you from room {} to room {} before you can even blink",
      current, next
    ));
    self.current_room = next;
    self.warning_messages();
    None
  } else {
    None
  }
}
```

Now we've got some actual behavior!  If we run into the wumpus or a bottomless pit, we die.  If we hit a bat, `current_room` will get a new random value, and we get a new set of warnings for our new location.

I'm having this function return an `Option<String>`.  We'll use this to decide ifg we want to end the game - a `None` will indicate the game should continue, and a `Some(string)` will trigger the end of the game.

Back in `lib.rs`, lets adjust our `update` function.  Adjust the `SwitchRoom` message handler:

```rust
SwitchRoom(target) => match self {
       Model::Playing(game) => {
         game.current_room = target;
         if let Some(msg) = game.move_effects() {
           *self = Model::Waiting(msg);
         };
       }
       _ => unreachable!(),
     },
```

Great!  Now we can wander around the maze with advance warning of all the horrors within.  Click around a while - you'll eventually die.  Isn't that fun?

Of course, one final step remains - we must be able to **shoot** this accursed beast.

First, let's create the message for it.  Open up `lib.rs` and add the new message type:

```rust
#[derive(Debug, Clone)]
pub enum Msg {
  StartGame,
  ShootArrow(u8),
  SwitchRoom(u8),
}
```

There are a few things we need to handle when the payer makes a shot.  If we hit the wumpus, the game will end and show a victory message.  If we missed and it was our last arrow - we're out of luck - the wumpus will eventually find you.  That's an immedaite loss.  Also, we're not necessarily subtle - each time we shoot there's a 75% chance we spook the Wumpus into an adjacant chamber.  If that adjacant champer happens to contain you, you're wumpus food.  Here's what that might look like in Rust - add this as a new match arm in your `update` function:

```rust
ShootArrow(target) => match self {
        Model::Playing(game) => {
          if game.wumpus == target {
            *self = Model::Waiting("With a sickening, satisfying thwack, your arrow finds its mark.  Wumpus for dinner tonight!  You win.".into());
          } else {
            game.arrows -= 1;
            game
              .messages
              .push("You arrow whistles aimlessly into the void".into());

            // If we exhausted our arrows, we lose
            if game.arrows == 0 {
              *self =
                Model::Waiting("You fired your very last arrow - you are now wumpus food".into());
            } else {
              // On each shot there's a 75% chance you scare the wumpus into an adjacant cell.
              let rand = js_rand(1, 4);
              if rand == 1 {
                game.messages.push(
                  "You listen quietly for any sign of movement - but the cave remains still."
                    .into(),
                );
              } else {
                game
                  .messages
                  .push("You hear a deafening roar - you've disturbed the wumpus!".into());
                let wumpus_exits = room_exits(game.wumpus).unwrap();
                let rand_idx = js_rand(0, 2);
                game.wumpus = wumpus_exits[rand_idx as usize];
                if game.wumpus == game.current_room {
                  *self = Model::Waiting("You scared the wumpus right on top of you.  Good going, mincemeat".into());
                }
              }
            }
          }
        }
```

Great!  Now all we need are some buttons to actually fire arrows.  Luckily, we've already got almost eveyrhting we need.  Over in `src/components/controls.rs`, lets make a little tweak to our `move_button` closure:

```rust
let move_button = |target: &u8| {
  use crate::Msg::*;
  let t = *target;
  html! {
      <div class="control-button",>
          <button onclick=|_| Msg::ButtonPressed(SwitchRoom(t)),>{&format!("Move to {}", target)}</button>
          <button onclick=|_| Msg::ButtonPressed(ShootArrow(t)),>{&format!("Shoot {}", target)}</button>
      </div>
  }
};
```

And that's the way the news goes!  Happy Wumpus huntin'.

Please show me if you improve this app!  I want to see what you come up with.