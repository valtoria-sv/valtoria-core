use criterion::{black_box, criterion_group, criterion_main, Criterion};
use your_project_name::blockchain::{Block, Blockchain};

fn add_block_benchmark(c: &mut Criterion) {
    c.bench_function("add_block", |b| {
        b.iter(|| {
            let mut blockchain = Blockchain::new();
            let block = Block::new(0, vec![], "genesis_hash".to_string());
            blockchain.add_block(block);
            black_box(blockchain);
        });
    });
}

fn blockchain_benchmark_group(c: &mut Criterion) {
    add_block_benchmark(c);
}

criterion_group!(benches, blockchain_benchmark_group);
criterion_main!(benches);
