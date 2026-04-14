use opeyml::genesis::*;
use opeyml::strict::*;

#[test]
fn test_suture_and_biopsy() {
    let mut doc = Value::Null;

    // Suture (=): Auto-vivify deep structures and assign values
    suture!(doc, .metadata.name = "nginx");
    suture!(doc, .metadata.replicas = 3);
    suture!(doc, .spec.containers[0].image = "nginx:latest");

    // Biopsy: Extraction without mutation
    assert_eq!(
        biopsy!(&doc, .metadata.name).unwrap().as_str().unwrap(),
        "nginx"
    );
    assert_eq!(
        biopsy!(&doc, .metadata.replicas).unwrap().as_i64().unwrap(),
        3
    );
    assert_eq!(
        biopsy!(&doc, .spec.containers[0].image)
            .unwrap()
            .as_str()
            .unwrap(),
        "nginx:latest"
    );

    // Safe extraction: Returns None for non-existent paths
    assert!(biopsy!(&doc, .spec.containers[1]).is_none());
}

#[test]
fn test_key_topology_resolution() {
    let mut doc = Value::Null;

    // Identifiers (.status -> "status")
    suture!(doc, .status = "ok");

    // Literals (Distinguishing between string and numeric keys)
    suture!(doc, ."http_code" = 200);
    suture!(doc, .404 = "not found");

    // Expressions (Keys as evaluated tokens)
    let dynamic_key = "dynamic";
    suture!(doc, .(dynamic_key) = "injected");
    suture!(doc, .(2 + 3) = "five");

    // Verification
    assert_eq!(biopsy!(&doc, .status).unwrap().as_str().unwrap(), "ok");
    assert_eq!(biopsy!(&doc, ."http_code").unwrap().as_i64().unwrap(), 200);
    assert_eq!(biopsy!(&doc, .404).unwrap().as_str().unwrap(), "not found");
    assert_eq!(
        biopsy!(&doc, ."dynamic").unwrap().as_str().unwrap(),
        "injected"
    );
    assert_eq!(biopsy!(&doc, .5).unwrap().as_str().unwrap(), "five");
}

#[test]
fn test_sequence_manipulation() {
    let mut doc = Value::Null;

    // Use graft! with = [ ... ] for appending to a sequence
    graft!(doc, .tags = ["rust"]);
    graft!(doc, .tags = ["macro"]);
    graft!(doc, .tags = ["yaml"]);

    assert_eq!(biopsy!(&doc, .tags[1]).unwrap().as_str().unwrap(), "macro");
    assert_eq!(biopsy!(&doc, .tags[2]).unwrap().as_str().unwrap(), "yaml");

    // Verify slicing/indexing via biopsy
    assert_eq!(biopsy!(&doc, .tags[0]).unwrap().as_str().unwrap(), "rust");
}

#[test]
fn test_error_scanning_and_excision() {
    let mut doc = Value::Null;
    suture!(doc, .system.kernel.version = "6.1.0");

    // Acquire: Precise Result-based extraction
    let acquire_result = acquire!(&doc, .system.kernel.arch);
    assert!(acquire_result.is_err());

    // Excise: Physical excision of nodes
    let removed = excise!(doc, .system.kernel.version).unwrap();
    assert_eq!(removed.as_str().unwrap(), "6.1.0");
    assert!(biopsy!(&doc, .system.kernel.version).is_none());
}

#[test]
fn test_mesh_deployment() {
    // Mesh: O(1) Multi-dimensional sequence deployment (In-place)
    let mut matrix = Value::Null;
    mesh!(matrix, [3][2]);

    assert!(matrix.is_sequence());
    let arr = matrix.as_sequence().unwrap();
    assert_eq!(arr.len(), 3);

    let inner_arr = arr[0].as_sequence().unwrap();
    assert_eq!(inner_arr.len(), 2);
    assert!(inner_arr[0].is_null());
}

#[test]
fn test_strict_and_force_suture() {
    let mut doc = Value::Null;
    suture!(doc, .config.mode = "init");

    // Strict Write (incise!): Only succeeds if the path already exists
    incise!(doc, .config.mode = "updated"); // Success
    incise!(doc, .config.debug = true); // Fails silently (path does not exist)

    assert_eq!(
        biopsy!(&doc, .config.mode).unwrap().as_str().unwrap(),
        "updated"
    );
    assert!(biopsy!(&doc, .config.debug).is_none());

    // Force Suture (force_suture!): Overwrites scalar values to construct the topology
    // Normally, assigning to .config.mode.deep would fail because .mode is already a String.
    force_suture!(doc, .config.mode = "destructive"); // Normal assign works

    // Utilize the optimized `={ }` syntax to instantly Forge and forcibly overwrite
    force_suture!(doc, .config.mode = { "deep" : "value" });

    assert!(biopsy!(&doc, .config.mode.deep).is_some());
}

#[test]
fn test_philosophical_completeness() {
    // 修正: serde_yaml::Value::Null ではなく、スコープ内の Value::Null を使用
    let mut doc = Value::Null;

    // ==========================================
    // 1. Graft overwrite test (Mapping over Scalar)
    // Proves that graft! overwrites scalars instead of merging,
    // while preserving its array/mapping merge semantics elsewhere.
    // ==========================================
    suture!(doc, .graft_target = "scalar_value");
    graft!(doc, .graft_target = { "new_key": 100 });

    assert_eq!(
        biopsy!(&doc, .graft_target."new_key")
            .unwrap()
            .as_i64()
            .unwrap(),
        100
    );

    // ==========================================
    // 2. Implant non-destructive test (Void filling)
    // Proves that implant! only fills Null spaces and NEVER
    // overwrites or mutates existing values.
    // ==========================================
    suture!(doc, .shield = "active");

    // Attempt to implant over an existing value (should be ignored)
    implant!(doc, .shield = "broken");
    assert_eq!(biopsy!(&doc, .shield).unwrap().as_str().unwrap(), "active");

    // Implant into a void (should auto-vivify and fill)
    implant!(doc, .void.hole = "filled");
    assert_eq!(
        biopsy!(&doc, .void.hole).unwrap().as_str().unwrap(),
        "filled"
    );
}
