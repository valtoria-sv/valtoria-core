use criterion::{black_box, criterion_group, criterion_main, Criterion};
use your_project_name::crypto::{generate_keys, hash, sign};

fn hash_benchmark(c: &mut Criterion) {
    let data = b"Benchmarking hash function";
    c.bench_function("hash", |b| {
        b.iter(|| {
            let result = hash(black_box(data));
            black_box(result);
        });
    });
}

fn signature_benchmark(c: &mut Criterion) {
    let (private_key, public_key) = generate_keys();
    let message = b"Benchmarking signature generation";

    c.bench_function("generate_signature", |b| {
        b.iter(|| {
            let signature = sign(black_box(&private_key), black_box(message));
            black_box(signature);
        });
    });
}

fn crypto_benchmark_group(c: &mut Criterion) {
    hash_benchmark(c);
    signature_benchmark(c);
}

criterion_group!(benches, crypto_benchmark_group);
criterion_main!(benches);
