extern crate ws;

use std::rc::Rc;
use std::cell::RefCell;
use ws::{listen, CloseCode, Handler, Handshake, Message, Sender};
use ws::util::Token;

struct Server {
    clients: Vec<Sender>,
}

impl Server {
    fn add_client(&mut self, sender: &Sender) {
        println!("New Connection {:?}", sender.token());
        self.clients.push(sender.clone());
    }

    fn remove_client(&mut self, sender: &Sender) -> Option<()> {
        let token = sender.token();
        let pos = match self.clients.iter().position(|x| x.token() == token) {
            Some(x) => x,
            None => return None,
        };

        println!("Lost Connection {:?}", sender.token());

        self.clients.remove(pos);
        Some(())
    }

    fn broadcast(&self, msg: Message, ignore_token: Option<Token>) {
        match ignore_token {
            Some(token) => for client in self.clients.iter().filter(
                    |&x| x.token() != token) {
                client.send(msg.clone()).unwrap();
            },
            None => for client in self.clients.iter() {
                client.send(msg.clone()).unwrap();
            },
        };
    }
}

type ServerRef = Rc<RefCell<Server>>;

#[derive(Clone)]
struct Client {
    server: ServerRef,
    sender: Sender,
}

impl Server {
    pub fn new() -> ServerRef {
        Rc::new(RefCell::new(Server {
            clients: Vec::new(),
        }))
    }
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        println!("Add client");
        self.server.borrow_mut().add_client(&self.sender);
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        self.server.borrow_mut().remove_client(&self.sender);
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let curr_token = self.sender.token();
        self.server.borrow().broadcast(msg, Some(curr_token));

        Ok(())
    }
}

fn main() {
    let server: ServerRef = Server::new();

    listen("127.0.0.1:3012", |sender| {
        Client {
            server: server.clone(),
            sender: sender,
        }
    }).unwrap()
}