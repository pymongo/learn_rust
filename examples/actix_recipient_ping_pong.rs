use actix::prelude::*;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "()")]
struct Ping {
    pub id: usize,
}

// Actor definition
struct PingPongGame {
    counter: usize,
    name: String,
    /// The `Recipient`(收信人) type allows to send one specific message to an Actor
    addr: Recipient<Ping>,
}

impl Actor for PingPongGame {
    type Context = Context<PingPongGame>;
}

// simple message handler for Ping message
impl Handler<Ping> for PingPongGame {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) {
        self.counter += 1;

        if self.counter > 10 {
            println!("[{0}] counter > 10", self.name);
            System::current().stop();
        } else {
            println!("[{0}] Ping received {1}", self.name, msg.id);

            // wait 100 nanos
            ctx.run_later(Duration::new(0, 100), move |act, _| {
                act.addr.do_send(Ping { id: msg.id + 1 }).unwrap();
            });
        }
    }
}

fn main() {
    let system = System::new();

    // To get a Recipient object, we need to use a different builder method
    // which will allow postponing actor creation
    let addr = PingPongGame::create(|ctx| {
        // 这步相当于建了一个图，两个Actor互连，各自的addr属性指向对方
        // now we can get an address of the first actor and create the second actor
        let addr = ctx.address();
        let addr2 = PingPongGame {
            counter: 0,
            name: String::from("Game 2"),
            addr: addr.recipient(),
        }
        .start();

        // let's start pings
        addr2.do_send(Ping { id: 10 });

        // now we can finally create first actor
        PingPongGame {
            counter: 0,
            name: String::from("Game 1"),
            addr: addr2.recipient(),
        }
    });
    dbg!(addr.recipient());

    system.run().unwrap();
}
