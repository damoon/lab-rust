use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};
use futures::{stream::StreamExt};

criterion_main!(benches);

criterion_group!(benches, bench_async_runtimes_and_channels);

fn bench_async_runtimes_and_channels(c: &mut Criterion) {
    let size: usize = 1_000_000;
    let chunks = 1_000;

    // async-std
    c.bench_with_input(BenchmarkId::new("use_async_std_channels", size), &size, |b, &size| {
        b.to_async(criterion::async_executor::AsyncStdExecutor).iter(|| use_async_std_channels(size, chunks));
    });

    c.bench_with_input(BenchmarkId::new("use_two_async_std_channels", size), &size, |b, &size| {
        b.to_async(criterion::async_executor::AsyncStdExecutor).iter(|| use_two_async_std_channels(size, chunks));
    });
    
}

enum Message {
    Event(String),
    FlushCommand,
}

struct Event {
    k: String,
}
struct Flush {}

async fn use_async_std_channels(size: usize, chunks: usize) {
    let (tx, rx) = async_std::channel::bounded(100);

    async_std::task::spawn(async move {
        for i in 1..=size {
            if i % chunks == 0 {
                if let Err(e) = tx.send(Message::FlushCommand).await {
                    panic!("send error: {}", e)
                }
                continue
            }

            if let Err(e) = tx.send(Message::Event("Hello, world.".to_string())).await {
                panic!("send error: {}", e)
            }
        }

        if let Err(e) = tx.send(Message::FlushCommand).await {
            panic!("send error: {}", e)
        }
    });

    while let Ok(message) = rx.recv().await {
        match message {
            Message::Event(e) => {
                black_box(e);
            },
            Message::FlushCommand => {}
        }
    }
}

async fn use_two_async_std_channels(size: usize, chunks: usize) {
    let (tx1, rx1) = async_std::channel::bounded(100);
    let (tx2, rx2) = async_std::channel::bounded(100);

    async_std::task::spawn(async move {
        for i in 1..=size {
            if i % chunks == 0 {
                if let Err(e) = tx2.send(Flush{}).await {
                    panic!("send error: {}", e)
                }
                continue
            }

            if let Err(e) = tx1.send(Event{k: "hwllo , wol".to_string()}).await {
                panic!("send error: {}", e)
            }
        }

        if let Err(e) = tx2.send(Flush{}).await {
            panic!("send error: {}", e)
        }
    });

    let mut rx1 = rx1.fuse();
    let mut rx2 = rx2.fuse();

    loop {
        futures::select!{
            e = rx1.next() => {
                match e {
                    Some(e) => {black_box(e.k);},
                    None => {},
                }
            }
            e = rx2.next() => {
                match e {
                    Some(e) => {black_box(e);},
                    None => {},
                }
            }
            complete => break,
        }
    }
}
