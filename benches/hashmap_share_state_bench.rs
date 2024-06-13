use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    iter::repeat_with,
    ops::Deref,
    sync::{atomic::AtomicU64, Arc},
};
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

    let keys = repeat_with(random_key)
        .take(n)
        .collect::<Vec<_>>();

    c.bench_function("no cache", |b| {
        b.iter(|| {
            let keys = keys.iter()
                .map(Deref::deref)
                .collect::<Vec<_>>();

            let mut map = HashMap::with_capacity(n);
            for _ in 0..repeat_count {
                for k in keys.iter().copied() {
                    map.insert(k, ());
                }
            }
        })
    });
    c.bench_function("cache key", |b| {
        b.iter(|| {
            let keys = keys.iter()
                .map(Deref::deref)
                .map(How::<_, DefaultHasher, Arc<AtomicU64>>::new)
                .collect::<Vec<_>>();

            #[allow(clippy::mutable_key_type)]
            let mut map = HashMap::with_capacity(n);
            for _ in 0..repeat_count {
                for k in keys.iter().cloned() {
                    map.insert(k, ());
                }
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
