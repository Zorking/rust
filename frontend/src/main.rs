extern crate stdweb;
extern crate hello_world;
use hello_world::chat;

// Shamelessly stolen from webplatform's TodoMVC example.



fn main() {
    stdweb::initialize();
    chat::run_chat();

    stdweb::event_loop();
}