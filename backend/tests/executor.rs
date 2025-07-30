use backend::{AsyncTcpStream, NetExecutor};
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::time::Duration;

#[test]
fn executor_responsive_multiple_connections() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let executor = NetExecutor::new();

    let mut c1 = TcpStream::connect(addr).unwrap();
    let mut c2 = TcpStream::connect(addr).unwrap();

    let (s1, _) = listener.accept().unwrap();
    let (s2, _) = listener.accept().unwrap();

    let mut a1 = AsyncTcpStream::new(s1, executor.clone()).unwrap();
    let mut a2 = AsyncTcpStream::new(s2, executor.clone()).unwrap();

    let (done_tx, done_rx) = channel();
    let (l1_tx, l1_rx) = channel();
    let (l2_tx, l2_rx) = channel();

    executor.clone().spawn(async move {
        let line = a1.read_line().await.unwrap();
        l1_tx.send(line).unwrap();
        std::future::pending::<()>().await;
    });

    executor.clone().spawn(async move {
        let line = a2.read_line().await.unwrap();
        l2_tx.send(line).unwrap();
        std::future::pending::<()>().await;
    });

    executor.clone().spawn(async move {
        done_tx.send(()).unwrap();
    });

    executor.run_for(Duration::from_millis(50));
    assert!(done_rx.try_recv().is_ok());

    c1.write_all(b"one\n").unwrap();
    c2.write_all(b"two\n").unwrap();

    executor.run_for(Duration::from_millis(50));

    assert_eq!(l1_rx.try_recv().unwrap().trim(), "one");
    assert_eq!(l2_rx.try_recv().unwrap().trim(), "two");
}
