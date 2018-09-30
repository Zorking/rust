extern crate yew;
extern crate counter;

use yew::prelude::*;
use counter::Model;

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}

//extern crate ws;
//
//use ws::listen;
//
//fn main() {
//  listen("127.0.0.1:3012", |out| {
//      move |msg| {
//         out.send(msg)
//      }
//  }).unwrap()
//}



//fn main() {
//    //TODO: can't use single quotes
//    println!("Hello world!");
//
//    let mut x = 5;
//    x += 10;
//    println!("{}", x);
//    {
//        let y = &mut x;
//        *y = add_one(*y);
//
//    }
//    println!("{}", x);
//    let f = add_one;
//    println!("{}", f(x)); //TODO: it will return 16 again for some reason
//    let a = [0;20];
//    let b = [12, 13];
//    println!("{}", a.len()); //TODO: can't just print full array here
//    println!("{}", b.len());
//    let qwert = ["qwert", "fgsfds"];
//    println!("{}", qwert[1]);
//
//    let slice = &a[5..10];
//    println!("{}", slice.len());
//
//    let (one, two, three) = (1, 2, 3);
//    println!("{}, {}, {}", one, two, three);
//
//    if x == 15 {
//        println!("No");
//    } else if x == 16 {
//        println!("True");
//    } else {
//        println!("Something is wrong");
//    }
//
//    let ternar = if x == 10 {9} else { 13 };
//    println!("{}", ternar);
//
//    for lopty_woop in 0..10 {
//        println!("{}", lopty_woop);
//    }
//}
//
//fn add_one(x: i32) -> i32 {
//    x + 1
//}