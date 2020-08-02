use actix::{Actor, Context, Message, System, Handler, WrapFuture, AsyncContext};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

struct Msg;

impl Message for Msg {
    type Result = ();
}

// 遗憾的是Rust目前没有async的Trait，Trait内部不允许出现async fn
impl Handler<Msg> for MyActor {
    type Result = ();

    fn handle(&mut self, _: Msg, ctx: &mut Context<Self>) {
        println!("In Handler<Msg> for MyActor");

        // async code block example 1:
        ctx.spawn(
            /* 执行顺序
            In Handler<Msg> for MyActor
            End of Handler<Msg> for MyActor
            In async code block
            */
            async {
                println!("In async code block");
            }
                .into_actor(self),
        );

        // 方法2，使用Actix同步函数内的异步模板: addr.send().into_actor(self).then(/*response回调*/).wait(ctx);
        // 因为addr.send()是异步的，想要读返回的数据就必须用异步的代码
        println!("End of Handler<Msg> for MyActor");
        System::current().stop();
    }
}

fn main() {
    let system = System::new("test");

    let addr = MyActor {}.start();
    // dbg!(std::mem::size_of_val(&addr)); // 24 Bytes
    // dbg!(std::mem::size_of_val(&addr.recipient())); // 16 Bytes

    let _req = addr.send(Msg {});

    system.run().unwrap();
}
