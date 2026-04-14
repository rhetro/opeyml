use opeyml::genesis::*;
use opeyml::strict::*;
use std::time::Instant;

#[test]
fn benchmark_opeyml_limit_break() {
    println!("\n🚀 --- Opeyml Performance Benchmark ---");

    // ==========================================
    // 1. Vertical Penetration (256 Levels)
    // ==========================================
    let mut data = Value::Null;
    let start_vp = Instant::now();
    let mut temp = &mut data;

    for i in 0..256 {
        let key = format!("v{}", i);
        // Dynamic keys are represented by .(expr).
        // Use = { } to statically and rapidly Forge an empty Mapping.
        suture!(*temp, .(key.as_str()) = { });

        // Move the mutable pointer to the deepest layer of the newly generated topology.
        let key_val = Value::from(key.as_str());
        temp = temp.as_mapping_mut().unwrap().get_mut(&key_val).unwrap();
    }

    // Engrave the proof of reaching the abyss (256th level).
    suture!(*temp, .abyss = "reached");

    let duration_vp = start_vp.elapsed();
    println!("📍 256-Level Penetration: {:?}", duration_vp);

    // ==========================================
    // 2. Massive Stitching (100,000 Sutures)
    // ==========================================
    // Execute 100,000 continuous Auto-vivifications and assignments from an empty document.
    let mut data_massive = Value::Null;
    let start_ms = Instant::now();

    for i in 0..100_000 {
        let k1 = (i % 100).to_string();
        let k2 = (i / 100 % 100).to_string();
        // Two-level nested Auto-vivification using dynamic keys.
        suture!(data_massive, .(k1.as_str()).(k2.as_str()) = i);
    }

    let duration_ms = start_ms.elapsed();
    println!("🧵 100,000 Massive Sutures: {:?}", duration_ms);

    // ==========================================
    // 3. Calculation for X (Total Score)
    // ==========================================
    let total_ops = 100_000 + 256;
    let total_time = duration_vp + duration_ms;
    let ops_per_sec = (total_ops as f64) / total_time.as_secs_f64();

    println!("-----------------------------------------");
    println!("🏆 FINAL SCORE: {:.2} operations/sec", ops_per_sec);
    println!("-----------------------------------------\n");
}

#[test]
fn benchmark_extreme_memory_and_allocation_limits() {
    use opeyml::genesis::*;
    use std::time::Instant;

    println!("\n🔥 --- Extreme Performance Limits Test ---");

    // ==========================================
    // 1. Extreme Spatial Deployment (1,000,000 Nodes Instant Allocation)
    // ==========================================
    // Tests the limit of `mesh!`. Allocating a 100x100x100 multi-dimensional array
    // instantly forces the allocation of 1,000,000 Null nodes.
    // This measures the overhead of O(1) syntax expansion against Rust's heap allocator.
    let mut matrix = Value::Null;
    let start_mesh = Instant::now();

    mesh!(matrix, [100][100][100]);

    let duration_mesh = start_mesh.elapsed();
    println!("🌌 1,000,000 Nodes Mesh Allocation: {:?}", duration_mesh);

    // Verify
    let seq = biopsy!(&matrix, [99][99]).unwrap().as_sequence().unwrap();
    assert_eq!(seq.len(), 100);

    // ==========================================
    // 2. Extreme Sequential Grafting (100,000 Array Appends)
    // ==========================================
    // Tests the limit of `graft!`. Appending 100,000 elements to a single sequence
    // triggers massive underlying vector reallocations and topology searches.
    let mut stream = Value::Null;
    let start_graft = Instant::now();

    for i in 0..100_000 {
        graft!(stream, .telemetry.events = [i]);
    }

    let duration_graft = start_graft.elapsed();
    println!(
        "🧵 100,000 Sequential Grafts (Appends): {:?}",
        duration_graft
    );

    // Verify
    assert_eq!(
        biopsy!(&stream, .telemetry.events)
            .unwrap()
            .as_sequence()
            .unwrap()
            .len(),
        100_000
    );

    // ==========================================
    // 3. Destructive Overwrite Limits (100,000 Type Overwrites)
    // ==========================================
    // Tests the garbage collection / memory drop limits when continuously
    // destroying and replacing a large data structure.
    let mut chimera = Value::Null;
    let start_overwrite = Instant::now();

    for i in 0..100_000 {
        // Continuously destroy the previous node and replace it
        if i % 2 == 0 {
            force_suture!(chimera, .core = { "status": "active" });
        } else {
            force_suture!(chimera, .core = "standby");
        }
    }

    let duration_overwrite = start_overwrite.elapsed();
    println!(
        "💥 100,000 Destructive Overwrites: {:?}",
        duration_overwrite
    );
    println!("-----------------------------------------\n");
}
