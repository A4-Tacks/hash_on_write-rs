use std::{iter::repeat_with, collections::{HashMap, hash_map::DefaultHasher}, rc::Rc, sync::atomic::AtomicU64};
use hash_on_write::How;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::random;

fn random_key() -> String {
    let len = random::<usize>() % 300;
    let mut str = String::with_capacity(len);
    for _ in 0..len {
        str.push(char::from(random::<u8>() % (127-32) + 32));
    }
    str
}

fn criterion_benchmark(c: &mut Criterion) {
    let n = 10000;
    let repeat_count = 50;

    c.bench_function("no cache", |b| {
        b.iter(|| {
            let keys = repeat_with(random_key)
                .take(n)
                .map(Rc::new)
                .collect::<Vec<_>>();

            let mut map = HashMap::with_capacity(n);
            for _ in 0..repeat_count {
                for k in &keys {
                    map.insert(k.clone(), ());
                }
            }
        })
    });
    c.bench_function("cache key", |b| {
        b.iter(|| {
            let wrapped_keys = repeat_with(random_key)
                .take(n)
                .map(How::<_, DefaultHasher, AtomicU64>::new)
                .map(Rc::new)
                .collect::<Vec<_>>();

            let mut map = HashMap::with_capacity(n);
            for _ in 0..repeat_count {
                for k in &wrapped_keys {
                    map.insert(k.clone(), ());
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
