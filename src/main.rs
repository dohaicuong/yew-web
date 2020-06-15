extern crate portfolio;
extern crate yew;

use portfolio::Model;
use yew::prelude::App;

fn main() {
  yew::initialize();
  let app: App<Model> = App::new();
  app.mount_to_body();
  yew::run_loop();
}
