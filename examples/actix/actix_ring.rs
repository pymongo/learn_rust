/*! Actix笔记
## 1. Actix Run-Time

fn main() {
    let system = System::new("test");
    // ...
    system.run();
}

或者使用actix_rt的宏

#[actix_web::main]
async fn main() {
}

### 1.1 Actor的生命周期

#### running -> stopping

1. Context::stop is called
2. all addresses to the actor get dropped(例如main函数结束时释放所以变量)
3. no event objects are registered in the context(setInterval/setTimeout?)

#### stopping -> running

1. create a new addr
2. add event(setInterval/setTimeout?)

### 1.2 Arbiters(仲裁官)

Arbiters hosts the environment where an actor runs

System::new creates an Arbiter for your actors to run inside

One Arbiter is in control of one thread with one event pool.

## 2. Construct new Actor

Spawn a new actor, 虽然Actor是个Trait，但笔记里对impl Actor的结构体简称为Actor

Actors cannot be referenced directly, only by their addresses

如果Actor已经运行了，则可以通过Actor的ctx返回Actor的addr

### 2.1 空结构体
空结构体可以用 `let addr = MyActor.start();`
其实这种情况完整写法跟2.2.2一样，都是MyActor{}.start();
所以Actor创建完之后就一定会被运行

### 2.2 有字段的结构体

#### 2.2.1 用MyActor::start()
let node_addr = Node::start(Node { id: 0, next: None });

#### 2.2.2 用MyActor {}.start()
let node_addr = Node { id: 0, next: None }.start();

#### 2.2.3 用MyActor::create()
let node_addr = Node::create(move |ctx| { Node { id: 0, next: None } });

## 如何修改Actor的结构体内的字段
例如链表往往创建时，self.next往往是None，需要一个setter方法去更新self.next?
根据Actor的设计思想，只能借助Actor的addr给Actor发一条消息，让Actor修改自己结构体的字段或返回字段的值

## Recipient

Recipient is a specialized version of an address that supports only one type of message

*/
use actix::prelude::*;

struct Node {
    id: usize,
    messages_receive_count: usize,
    next_recipient: Recipient<Msg>,
}

impl Actor for Node {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("start: Node({})", self.id);
    }
}

struct Msg {
    from_actor_id: usize,
}

/* Same as:
#[derive(Message)]
#[rtype(result = "()")]
struct Msg {
*/
impl Message for Msg {
    type Result = ();
}

impl Handler<Msg> for Node {
    // Message response type
    type Result = ();

    // &mut Context<Self>参数用于run_later或run_interval
    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) {
        if self.messages_receive_count > 2 {
            println!("Actor {} reached message limit of 2", self.id);
            System::current().stop();
            return;
        }
        println!(
            "[{}->{}] Actor {} received message",
            msg.from_actor_id, self.id, self.id,
        );
        self.messages_receive_count += 1;
        self.next_recipient
            .do_send(Msg {
                from_actor_id: self.id,
            })
            .unwrap();
    }
}

/*
N个链表节点围成一圈，从节点0开始往next_node发一条信息，信息在环内转两圈后结束
*/
fn main() {
    let system = System::new();
    let second_node = Node::create(move |ctx| {
        let first_addr = ctx.address();
        let mut successor_addr = Node {
            id: 1,
            messages_receive_count: 0,
            next_recipient: first_addr.recipient(),
        }
        .start();

        for id in (3..=5).rev() {
            successor_addr = Node {
                id,
                messages_receive_count: 0,
                next_recipient: successor_addr.recipient(),
            }
            .start();
        }

        // dummyHead?
        Node {
            id: 2,
            messages_receive_count: 0,
            next_recipient: successor_addr.recipient(),
        }
    });
    let _msg_req = second_node.send(Msg { from_actor_id: 2 });
    /*
    start: Node(1)
    start: Node(5)
    start: Node(4)
    start: Node(3)
    start: Node(2)
    [2->2] Actor 2 received message
    [2->3] Actor 3 received message
    [3->4] Actor 4 received message
    [4->5] Actor 5 received message
    [5->1] Actor 1 received message
    [1->2] Actor 2 received message
    [2->3] Actor 3 received message
    [3->4] Actor 4 received message
    [4->5] Actor 5 received message
    [5->1] Actor 1 received message
    */
    system.run().unwrap();
}
