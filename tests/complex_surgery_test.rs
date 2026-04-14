use opeyml::genesis::*;
use opeyml::strict::*;

#[test]
fn test_kubernetes_level_deep_surgery() {
    // 1. Complex initial state (Simulating a deep K8s Deployment tree)
    let yaml_str = r#"
apiVersion: apps/v1
kind: Deployment
metadata:
  name: the-empty-os-gateway
spec:
  replicas: 2
  template:
    spec:
      containers:
        - name: gateway
          image: ordestra/gateway:v1
"#;

    let mut doc: Value = serde_yaml::from_str(yaml_str).unwrap();

    // ==========================================
    // 🔪 Surgery 1: Array skipping and deep Auto-vivification
    // ==========================================
    suture!(doc, .spec.template.spec.containers[1].name = "metrics-sidecar");
    suture!(doc, .spec.template.spec.containers[1].image = "ordestra/metrics:latest");
    suture!(doc, .spec.template.spec.containers[1].ports[0].containerPort = 9090);

    // ==========================================
    // 🔪 Surgery 2: Sequential appending via graft! (= [...])
    // ==========================================
    // Directly pushing parsed YAML objects into the sequence.
    let env_1: Value = serde_yaml::from_str(r#"{ "name": "RUST_LOG", "value": "debug" }"#).unwrap();
    let env_2: Value =
        serde_yaml::from_str(r#"{ "name": "MAX_LATENCY", "value": "10ms" }"#).unwrap();
    graft!(doc, .spec.template.spec.containers[0].env = [env_1]);
    graft!(doc, .spec.template.spec.containers[0].env = [env_2]);

    // ==========================================
    // 🔪 Surgery 3: Injection with heterogeneous keys (Identifiers, Literals, Expressions)
    // ==========================================
    let dynamic_trace_id = "trace_id";
    suture!(doc, .metadata.annotations.(dynamic_trace_id) = "0xdeadbeef");
    suture!(doc, .openapi.responses.200.description = "OK");
    suture!(doc, .openapi.responses.404.description = "Not Found");
    suture!(doc, .openapi.responses.500.strict = true);

    // ==========================================
    // 🔪 Surgery 4: Strict Write (incise!) vs Standard Assign (suture!)
    // ==========================================
    suture!(doc, .spec.strategy.type = "RollingUpdate");

    // Strict write: Fails silently because `.spec.strategy.timeout` does not exist yet.
    incise!(doc, .spec.strategy.timeout = "30s");

    // ==========================================
    // 🔬 Verification via Zero-Allocation Biopsy
    // ==========================================

    // Verify Surgery 1
    assert_eq!(
        biopsy!(&doc, .spec.template.spec.containers[1].name)
            .unwrap()
            .as_str()
            .unwrap(),
        "metrics-sidecar"
    );
    assert_eq!(
        biopsy!(&doc, .spec.template.spec.containers[1].ports[0].containerPort)
            .unwrap()
            .as_i64()
            .unwrap(),
        9090
    );

    // Verify Surgery 2
    assert_eq!(
        biopsy!(&doc, .spec.template.spec.containers[0].env[0].name)
            .unwrap()
            .as_str()
            .unwrap(),
        "RUST_LOG"
    );
    assert_eq!(
        biopsy!(&doc, .spec.template.spec.containers[0].env[1].value)
            .unwrap()
            .as_str()
            .unwrap(),
        "10ms"
    );

    // Verify Surgery 3
    assert_eq!(
        biopsy!(&doc, .metadata.annotations."trace_id")
            .unwrap()
            .as_str()
            .unwrap(),
        "0xdeadbeef"
    );
    assert_eq!(
        biopsy!(&doc, .openapi.responses.200.description)
            .unwrap()
            .as_str()
            .unwrap(),
        "OK"
    );
    assert_eq!(
        biopsy!(&doc, .openapi.responses.500.strict)
            .unwrap()
            .as_bool()
            .unwrap(),
        true
    );

    // Verify Surgery 4
    assert_eq!(
        biopsy!(&doc, .spec.strategy.type)
            .unwrap()
            .as_str()
            .unwrap(),
        "RollingUpdate"
    );
    assert!(biopsy!(&doc, .spec.strategy.timeout).is_none()); // Proves Strict Write (incise!) successfully protected the tree

    // ==========================================
    // 💣 Excision (Excise) & Tracking (Acquire)
    // ==========================================

    // Excise a container
    let removed = excise!(doc, .spec.template.spec.containers[1]).unwrap();
    assert_eq!(
        removed
            .as_mapping()
            .unwrap()
            .get(&Value::from("name"))
            .unwrap()
            .as_str()
            .unwrap(),
        "metrics-sidecar"
    );

    // Acquire should now return an Error that the path is severed at [1]
    let err = acquire!(&doc, .spec.template.spec.containers[1].ports[0]).unwrap_err();
    assert!(
        format!("{:?}", err).contains("1"),
        "Acquire correctly identifies the amputation point"
    );
}

#[test]
fn test_lexer_boundary_and_heterogeneous_keys() {
    // 修正: serde_yaml::Value::Null ではなく、スコープ内の Value::Null を使用
    let mut hell = Value::Null;

    // 1. Preventing Rust lexer float misinterpretation (. 1 . 2 vs .1.2)
    suture!(hell, . 1 . 2 . 3 ."4". 5 = "reached");

    // 2. Extending the structure safely
    suture!(hell, . 1 . 2 . 3 ."4".extra = "data");

    // 3. Sequential gap injection (Targeting index 10 directly)
    suture!(hell, .nodes[10].id = "edge-node");

    // 4. Dynamic access via expressions and Append (graft!)
    let key = "target";
    graft!(hell, .metadata.(key).status = ["initialized"]);

    // 🔬 Observation (Proving the topology integrity)
    assert_eq!(
        biopsy!(&hell, . 1 . 2 . 3 ."4". 5)
            .unwrap()
            .as_str()
            .unwrap(),
        "reached"
    );
    assert_eq!(
        biopsy!(&hell, . 1 . 2 . 3 ."4".extra)
            .unwrap()
            .as_str()
            .unwrap(),
        "data"
    );
    assert_eq!(
        biopsy!(&hell, .nodes[10].id).unwrap().as_str().unwrap(),
        "edge-node"
    );
    assert_eq!(
        biopsy!(&hell, .metadata."target".status[0])
            .unwrap()
            .as_str()
            .unwrap(),
        "initialized"
    );

    // .len() is a Rust method, evaluated outside the macro
    assert_eq!(
        biopsy!(&hell, .nodes).unwrap().as_sequence().unwrap().len(),
        11
    );
}

#[test]
fn test_force_auto_vivification_destruction() {
    let mut doc = Value::Null;

    // 1. Create a scalar at the node
    suture!(doc, .network.status = "connected");
    assert_eq!(
        biopsy!(&doc, .network.status).unwrap().as_str().unwrap(),
        "connected"
    );

    // 2. Safe Auto-vivification (=) is blocked by the scalar.
    // It will refuse to destroy "connected" to build the path, so this does nothing.
    suture!(doc, .network.status.latency = "1ms");
    assert!(biopsy!(&doc, .network.status.latency).is_none());
    assert_eq!(
        biopsy!(&doc, .network.status).unwrap().as_str().unwrap(),
        "connected"
    ); // Still intact

    // 3. Destructive Assignment (force_suture!) violently overwrites the scalar "connected"
    // into a Mapping to construct the requested topology.
    force_suture!(doc, .network.status.latency = "1ms");

    // The scalar is dead. The mapping is forged.
    assert_eq!(
        biopsy!(&doc, .network.status.latency)
            .unwrap()
            .as_str()
            .unwrap(),
        "1ms"
    );
}
