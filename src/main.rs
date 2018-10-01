extern crate yew;
extern crate hello_world;

use yew::prelude::*;
use hello_world::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
