extern crate yew;
extern crate hello_world;

use yew::prelude::*;
use hello_world::Model;

//pub struct Qwe {
//    qqqq: Vec<String::new()]>
//}

fn main() {
//    let q = Qwe{qqqq:Vec::new()};
//    let mut v = q.qqqq;
//    v.push("qwe");
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
//    let q = Vec::new();foo
//    let n = vec![1, 2, 3];
//
//    let renderer = |inx| {inx + 1};
//    for q in n.into_iter().map(renderer) {
//        println!("{}", q);
//    }

}
