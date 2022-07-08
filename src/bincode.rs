// bincode 的最大缺点: 不支持 serde(flatten)


#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Req {
    id: u32
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Rsp {
    id: u32
}

#[cfg(test)]
const PORT: u16 = 4444;

// #[tokio::test]
#[cfg(test)]
async fn server() {

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct Req {
        id: u32
    }

    let listener = tokio::net::TcpListener::bind(std::net::SocketAddr::from(([0,0,0,0], PORT))).await.unwrap();
    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut stream = async_bincode::tokio::AsyncBincodeStream::<_, Req, Rsp, _>::from(stream);
            let (mut read, mut write) = stream.tcp_split();
            // while let Some(req) = futures::StreamExt::next(&mut read).await {
            //     let req = req.unwrap();
            //     let rsp = Rsp { id: req.id + 1 };
            //     dbg!(&req, &rsp);
            //     futures::SinkExt::send(&mut write, rsp).await.unwrap();
            // }
            use futures::prelude::*;
            while let Some(req) = read.next().await {
                let req = req.unwrap();
                let rsp = Rsp { id: req.id + 1 };
                dbg!(&req, &rsp);
                futures::SinkExt::send(&mut write, rsp).await.unwrap();
            }
        });
    }
}

#[cfg(test)]
// #[tokio::test]
async fn client() {
    let stream = tokio::net::TcpStream::connect(std::net::SocketAddr::from(([0,0,0,0], PORT))).await.unwrap();
    let mut stream = async_bincode::tokio::AsyncBincodeStream::<_, Rsp, Req, _>::from(stream);
    let (mut read, mut write) = stream.tcp_split();
    let mut id = 0;
    while id < 5 {
        let req = Req { id };
        dbg!(&req);
        futures::SinkExt::send(&mut write, req).await.unwrap();
        let rsp = futures::StreamExt::next(&mut read).await.unwrap().unwrap();
        dbg!(&rsp);
        id = rsp.id;
    }
}

#[cfg(test)]
// #[tokio::test]
async fn it_works() {
    use async_bincode::tokio::*;
    use futures::prelude::*;
    use std::net::IpAddr;

    #[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
    struct Req {
        req_id: u32
    }
    
    #[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
    struct Rsp {
        rsp_id: u32
    }

    let echo = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = echo.local_addr().unwrap();

    let handle = tokio::spawn(async move {
        let (stream, addr) = echo.accept().await.unwrap();
        dbg!(addr);
        let mut stream = AsyncBincodeStream::<_, Req, Rsp, _>::from(stream).for_async();
        let (mut r, mut w) = stream.tcp_split();
        while let Some(xx) =  r.next().await {
            let req = xx.unwrap();
            w.send(Rsp::default()).await.unwrap();
        }
        dbg!("EOF");
        // r.forward(w).await.unwrap();
    });

    let client = tokio::net::TcpStream::connect(&addr).await.unwrap();
    let mut client = AsyncBincodeStream::<_, Rsp, Req, _>::from(client).for_async();
    client.send(Req::default()).await.unwrap();
    assert_eq!(client.next().await.unwrap().unwrap().rsp_id, 0);
    dbg!("send/recv success");

    // client.send(44).await.unwrap();
    // assert_eq!(client.next().await.unwrap().unwrap(), 44);

    drop(client);
    handle.await.unwrap();
}

#[test]
fn run_server() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_io().build().unwrap();
    rt.block_on(server());
}

#[test]
fn run_client() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_io().build().unwrap();
    rt.block_on(it_works());
}
