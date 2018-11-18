# Hunt the Wumpus

In this post we'll walk through recreating the classic [Hunt the Wumpus](https://en.wikipedia.org/wiki/Hunt_the_Wumpus) game in [Yew](https://github.com/DenisKolodin/yew).  The original was played at the command line, we're going to use a webpage.

This is a beginner-level tutorial - though it's helpful to be familiar with reading Rust.

## Setup

You'll need a nightly Rust compiler.  See [rustup](https://rustup.rs/) to get started if you need.  You'll also need [`cargo-web`](https://github.com/koute/cargo-web): `cargo install cargo-web`.  Once you have that installed, navigate to your project directory and issue `cargo new hunt-the-wumpus` at the terminal.  Open that folder in the text editor of your choice.  We're going to start by adding the basic outline of the app and build pipeline - enough to get everything compiling and running.

First thing's first - we want to use the built-in Rust target.  Issue the following commands:

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
yew = { git = "https://github.com/DenisKolodin/yew" }
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
$ touch scss/
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

Now we can start modelling the logic.  First thing's first - let's define the cave.  The traditional game is played in a cave where each room is a vertex of a regular dodecahedron.  From each room, we are connected to exactly three other rooms.

We'll use a function to map room IDs to exits.  Place the following in `lib.rs`, above your `Model` declaration:

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

Yes, this was relatively annoying to make.  You're welcome.

Let's store the player's current location in the `Model`:

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

Now we can start adding to our UI.  We'll make a new component that will be responsible for rendering the controls.  I like keeping all of these in a folder:

```
$ mkdir src/components
$ touch src/components/controls.rs
```

We'll start with a basrebones component:

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

Unlike our top-level component, this one accepts some props - we're going to pass in the exits to the room our player is in.  A couple of "gotchas" - take a look at the `html!` macro in the `Renderable` impl block.  We're attaching two classes to the top-level `div` - to do so, you need to wrap them up in a tuple like shown.  Also, if you're using an attribute in your tag, you need to include that trailing comma for the macro to work.  If you don't, you might end up with a very dense error message - check for these commas before panicking.  Rust macros don't necessarily yeild themselves towards useful error info - one major drawback of the tech at this point in time.

Also of not - we *must* provide a `Default` impl for our `Props`.  I'm just setting it to `[0, 0, 0]`.

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

Just what we asked for!  Before we get too far into the logic, let's give ourselves something resembling a layout.  This is just going to be a skeleton - I'm no CSS guru.  Feel free to make this whatever you like, this should just get you started.

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
      }
    }
  }
}
```

Let's also go ahead and take the opportunity to break out Stats out into their own component.  Make a new file `src/components/stats.rs`:

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

Great!  Well, at least Not Terrible!  Our next order of business is moving around the cave.