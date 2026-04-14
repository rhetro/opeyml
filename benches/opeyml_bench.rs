use criterion::{black_box, criterion_group, criterion_main, Criterion};
use opeyml::genesis::*;
use serde_yaml::Value;

fn benchmark_deep_penetration(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deep Penetration");

    group.bench_function("opeyml_suture", |b| {
        b.iter(|| {
            let mut doc = Value::Null;
            // O(1) compile-time path routing
            suture!(doc, .level1.level2.level3.level4 = black_box("payload"));
            black_box(doc);
        });
    });

    // Optional: Add a baseline bench using pure serde_yaml
    // to prove the absolute superiority of opeyml's DSL.
    /*
    group.bench_function("serde_yaml_manual", |b| {
        b.iter(|| {
            // Write the verbose manual equivalent here
        });
    });
    */

    group.finish();
}

criterion_group!(benches, benchmark_deep_penetration);
criterion_main!(benches);
