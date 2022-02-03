use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lab::{fibonacci_slow, fibonacci_fast, fibonacci_realy_fast};
use std::{
    fs::File,
    io::{Read, Write},
    time::Instant,
};
use tokio::task::{self, JoinHandle};
//use tokio::runtime::Runtime;
use criterion::async_executor::FuturesExecutor;
use criterion::async_executor::AsyncExecutor;
use tokio::sync::mpsc;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("o(n^2) fib 20", |b| b.iter(|| fibonacci_slow(black_box(20))));
    c.bench_function("o(n) fib 120", |b| b.iter(|| fibonacci_fast(black_box(120))));
    c.bench_function("o(1) fib 1000", |b| b.iter(|| fibonacci_realy_fast(black_box(1000))));
}

async fn do_something(size: usize) {
    
    let handles: Vec<JoinHandle<_>> = (0..size)
        .map(|_| {
            tokio::spawn(async move {
                let mut buffer = [0; 10];
                {

                    task::block_in_place(move || {
                        let mut dev_urandom = File::open("/dev/urandom").unwrap();
                        dev_urandom.read(&mut buffer).unwrap();
                    });
                }
                task::block_in_place(move || {
                    let mut dev_null = File::create("/dev/null").unwrap();
                    dev_null.write(&mut buffer).unwrap();
                });
            })
        })
        .collect();
    for handle in handles {
        handle.await.unwrap();
    }
}

enum Message {
    Event(String),
    FlushCommand,
}

async fn use_tokio_channels(size: usize) {
    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=size {
            if i % 1000 == 0 {
                if let Err(e) = tx.send(Message::FlushCommand).await {
                    panic!("send error: {}", e)
                }
                continue
            }

            if let Err(e) = tx.send(Message::Event("Hello, world.".to_string())).await {
                panic!("send error: {}", e)
            }

        }
    });

    while let Some(message) = rx.recv().await {
        match message {
            Message::Event(e) => {
                black_box(e);
            },
            Message::FlushCommand => {}
        }
    }
}

struct Event {
    k: &'static str,
}
struct Flush {}

async fn use_two_tokio_channels(size: usize) {
    let (tx1, mut rx1) = mpsc::channel(100);
    let (tx2, mut rx2) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 1..=size {
            if i % 1000 == 0 {
                if let Err(e) = tx2.send(Flush{}).await {
                    panic!("send error: {}", e)
                }
                continue
            }

            if let Err(e) = tx1.send(Event{k: "hwllo , wol"}).await {
                panic!("send error: {}", e)
            }
        }
    });

    tokio::select!{
        Some(e) = rx1.recv() => {
            black_box(e);
        }
        Some(e) = rx2.recv() => {
            black_box(e);
        }
        else => {
            // Both channels closed
            return
        }
    }
}

fn from_elem(c: &mut Criterion) {
    let size: usize = 1_000;
    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| use_two_tokio_channels(s));
    });

    let size: usize = 1_000_000;
    c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
        // Insert a call to `to_async` to convert the bencher to async mode.
        // The timing loops are the same as with the normal bencher.
        b.to_async(tokio::runtime::Runtime::new().unwrap()).iter(|| use_two_tokio_channels(s));
    });
}

criterion_group!(benches, from_elem);
criterion_main!(benches);
