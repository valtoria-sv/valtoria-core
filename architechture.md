monero-rust/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── bin/
│   │   ├── node.rs
│   │   └── wallet_cli.rs
│   ├── blockchain/
│   │   ├── mod.rs
│   │   ├── block.rs
│   │   ├── transaction.rs
│   │   └── chain.rs
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── keys.rs
│   │   ├── ring_signature.rs
│   │   └── stealth_address.rs
│   ├── networking/
│   │   ├── mod.rs
│   │   ├── peer.rs
│   │   └── protocol.rs
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── pow.rs
│   │   └── validator.rs
│   ├── wallet/
│   │   ├── mod.rs
│   │   ├── account.rs
│   │   └── transaction_builder.rs
│   ├── mining/
│   │   ├── mod.rs
│   │   └── miner.rs
│   ├── storage/
│   │   ├── mod.rs
│   │   └── database.rs
│   └── api/
│       ├── mod.rs
│       ├── rpc.rs
│       └── rest.rs
├── tests/
│   ├── integration_tests/
│   │   ├── blockchain_tests.rs
│   │   ├── network_tests.rs
│   │   └── wallet_tests.rs
│   └── fuzzing/
│       ├── crypto_fuzzer.rs
│       └── transaction_fuzzer.rs
├── benches/
│   ├── crypto_benchmarks.rs
│   └── blockchain_benchmarks.rs
├── docs/
│   ├── architecture.md
│   ├── api_reference.md
│   └── development_guide.md
└── scripts/
    ├── setup.sh
    └── run_tests.sh
