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
        <footer>
                <a href="https://github.com/deciduously/hunt-the-wumpus",>{"source"}</a>
        </footer>
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
[source](https://github.com/deciduously/hunt-the-wumpus)

We're up and running!