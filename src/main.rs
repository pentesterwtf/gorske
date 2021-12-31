#![recursion_limit = "256"]

mod text_input;

mod app;
mod gorske;

use app::App;

fn main() {
    yew::start_app::<App>();
}
