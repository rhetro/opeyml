use opeyml::genesis::*;
use opeyml::strict::*;

#[test]
fn test_file_load_and_surgery() {
    // 1. Prepare YAML payload (inline to ensure test independence)
    let yaml_str = r#"
project: "The Empty OS"
metadata:
  tags:
    - rust
    - yaml
  active: false
spec:
  replicas: 3
"#;

    // The initial serde_yaml parsing allocates memory, but
    // all subsequent path traversals are resolved at compile time with zero allocation.
    let mut doc: Value = serde_yaml::from_str(yaml_str).expect("Failed to parse YAML");

    // 2. Observe existing values (Strict Biopsy)
    assert_eq!(
        biopsy!(&doc, .project).unwrap().as_str().unwrap(),
        "The Empty OS"
    );
    assert_eq!(
        biopsy!(&doc, .metadata.tags[0]).unwrap().as_str().unwrap(),
        "rust"
    );

    // 3. Safe suture & mutation (Suture =)
    // Reaches the target directly as binary instructions, entirely bypassing string parsing.
    suture!(doc, .spec.replicas = 5);
    suture!(doc, .metadata.active = true);

    // 4. Structural appending and expansion (graft! instead of <<)
    // Append to sequence
    graft!(doc, .metadata.tags = ["opeyml"]);

    // Inject new properties into an object (Auto-vivification)
    suture!(doc, .spec.port = 8080);
    suture!(doc, .spec.protocol = "TCP");

    // 5. Final Verification
    // Verify updated values
    assert_eq!(biopsy!(&doc, .spec.replicas).unwrap().as_i64().unwrap(), 5);
    assert_eq!(
        biopsy!(&doc, .metadata.active).unwrap().as_bool().unwrap(),
        true
    );

    // Verify appended elements
    assert_eq!(
        biopsy!(&doc, .metadata.tags[2]).unwrap().as_str().unwrap(),
        "opeyml"
    );
    assert_eq!(biopsy!(&doc, .spec.port).unwrap().as_i64().unwrap(), 8080);
    assert_eq!(
        biopsy!(&doc, .spec.protocol).unwrap().as_str().unwrap(),
        "TCP"
    );

    // 6. Verify Advanced Operations: Error tracking (acquire!) and Excision (excise!)
    // Acquire a non-existent path and ensure the severed node is accurately reported.
    let err = acquire!(&doc, .spec.not_found.item).unwrap_err();
    assert!(
        format!("{:?}", err).contains("not_found"),
        "Error should point exactly to the severed path"
    );

    // Physically amputate "yaml" (index 1) from the tags array.
    let removed = excise!(doc, .metadata.tags[1]).unwrap();
    assert_eq!(removed.as_str().unwrap(), "yaml");

    // 7. (Optional) Serialize the mutated result back to a string
    let output = serde_yaml::to_string(&doc).unwrap();
    // println!("--- Mutated YAML ---\n{}", output);

    // String-level verification of the mutated topology
    assert!(output.contains("replicas: 5"));
    assert!(output.contains("opeyml"));
    assert!(!output.contains("- yaml")); // Ensure the excised element is eradicated
}
