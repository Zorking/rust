#[macro_use]
extern crate yew;

use yew::prelude::*;
use yew::services::ConsoleService;

pub struct Model {
    console: ConsoleService,
    value: i32,
    messages: Vec<i32>,
}

pub enum Msg {
    GotInput(i32),
    Clicked,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            console: ConsoleService::new(),
            value: 0,
            messages: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GotInput(new_value) => {
                self.value = new_value;
            }
            Msg::Clicked => {
//                let y = *self.value;
                self.messages.push(self.value);
//                let x = self.messages.last();
//                let s: String = x.to_string();
                let s = "qwe";
                self.console.log(&s);
                println!("{}", s);
                self.value = 0;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <div>
                    <textarea rows=5,
                        value=&self.value,
                        oninput=|e| Msg::GotInput(e.value.parse::<i32>().unwrap()),
                        placeholder="placeholder",>
                    </textarea>
                     <button onclick=|_| Msg::Clicked,>{ "Send" }</button>
                </div>
                <ul>
                     { self.view_cols() }
                </ul>
            </div>
        }
    }
}

impl Model {
    fn view_cols(&self) -> Html<Self> {
        let mut veee = self.messages.to_vec();
        let render = |idx| html! {
            <li>{ idx }</li>
        };
        html! { // We use a fragment directly
            { for &mut  veee.into_iter().map(render) }
        }
    }
}
