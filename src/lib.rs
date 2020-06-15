#![recursion_limit="256"]

extern crate stdweb;
// #[macro_use]
extern crate yew;

use yew::prelude::*;

pub struct Model {}

#[derive(Debug, Clone)]
pub enum Msg {}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Model {}
  }

  fn update(&mut self, _msg: Self::Message) -> ShouldRender {
    true
  }
}

impl Renderable<Model> for Model {
  fn view(&self) -> Html<Self> {
    html! {
      <div class="App",>
        <div class="bg-indigo-900 text-center py-4 lg:px-4",>
          <div class="p-2 bg-indigo-800 items-center text-indigo-100 leading-none lg:rounded-full flex lg:inline-flex", role="alert",>
            <span class="flex rounded-full bg-indigo-500 uppercase px-2 py-1 text-xs font-bold mr-3",>
              {"New"}
            </span>
            <span class="font-semibold mr-2 text-left flex-auto",>
              {"Get the coolest t-shirts from our brand new store"}
            </span>
            <svg
              class="fill-current opacity-75 h-4 w-4",
              xmlns="http://www.w3.org/2000/svg",
              viewBox="0 0 20 20",
            >
              <path d="M12.95 10.707l.707-.707L8 4.343 6.586 5.757 10.828 10l-4.242 4.243L8 15.657l4.95-4.95z",></path>
            </svg>
          </div>
        </div>
      </div>
    }
  }
}