use opeyml::genesis::*;
use opeyml::strict::*;

#[test]
fn test_chimera_type_morphing() {
    // [Extreme Test 1: Continuous Type Destruction and Memory Safety]
    // Tests whether memory leaks or invalid pointer references occur (verifying Rust's Drop works correctly)
    // by violently overwriting the node type using force_suture! in the sequence:
    // Null -> Mapping -> Scalar -> Sequence -> Mapping.

    let mut doc = Value::Null;

    // 1. Null -> Mapping
    suture!(doc, .entity.core = "initial");
    assert!(biopsy!(&doc, .entity).unwrap().is_mapping());

    // 2. Mapping -> Scalar (Destructive)
    force_suture!(doc, .entity = "annihilated");
    assert!(biopsy!(&doc, .entity).unwrap().is_string());

    // 3. Scalar -> Sequence (Destructive)
    force_suture!(doc, .entity[0] = "reborn");
    assert!(biopsy!(&doc, .entity).unwrap().is_sequence());

    // 4. Sequence -> Mapping (Destructive)
    force_suture!(doc, .entity.new_core = "mutated");
    assert!(biopsy!(&doc, .entity).unwrap().is_mapping());

    // Validate survival
    assert_eq!(
        biopsy!(&doc, .entity.new_core).unwrap().as_str().unwrap(),
        "mutated"
    );
}

#[test]
fn test_astronomical_sequence_gap() {
    // [Extreme Test 2: Auto-vivification of an Astronomical Sequence Gap]
    // Accesses and assigns to index `9999` directly from an empty (Null) state.
    // Tests whether the intermediate range 0..9998 is correctly filled with Null,
    // and the sequence is expanded without panicking.

    let mut doc = Value::Null;

    // Target an absurd index directly from Null
    suture!(doc, .universe.galaxies[9999] = "Milky Way");

    let seq = biopsy!(&doc, .universe.galaxies)
        .unwrap()
        .as_sequence()
        .unwrap();
    assert_eq!(seq.len(), 10000);
    assert!(seq[0].is_null());
    assert!(seq[9998].is_null());
    assert_eq!(seq[9999].as_str().unwrap(), "Milky Way");

    // Excise the absurd distance
    let removed = excise!(doc, .universe.galaxies[9999]).unwrap();
    assert_eq!(removed.as_str().unwrap(), "Milky Way");
}

#[test]
fn test_extreme_dynamic_expressions() {
    // [Extreme Test 3: Complex Dynamic Expression Evaluation during Macro Expansion]
    // Injects a complex Rust block expression containing variable declarations and function calls
    // inside the .(expr) block to test if the AST parsing breaks.

    let mut doc = Value::Null;

    // Injecting heavy logic inside the dynamic key syntax
    suture!(doc, .computed.(
        {
            let a = 10;
            let b = 20;
            format!("key_{}", a + b)
        }
    ) = "success");

    assert_eq!(
        biopsy!(&doc, .computed."key_30").unwrap().as_str().unwrap(),
        "success"
    );

    // Nested dynamic evaluations
    let base = "level";
    suture!(doc, .(format!("{}_1", base)).(format!("{}_2", base)) = "deep");
    assert_eq!(
        biopsy!(&doc, ."level_1"."level_2")
            .unwrap()
            .as_str()
            .unwrap(),
        "deep"
    );
}

#[test]
fn test_chaos_string_keys() {
    // [Extreme Test 4: Routing with Chaotic String Keys]
    // Tests whether keys that are normally unparseable as properties in OOP languages,
    // such as empty strings (""), newlines, and emojis, can be successfully resolved as dynamic/static paths.

    let mut doc = Value::Null;

    // Empty string key (Valid in YAML)
    suture!(doc, .("") = "empty_key_value");

    // Newlines and emojis
    suture!(doc, .("multi\nline").("🔥") = "chaos");

    assert_eq!(
        biopsy!(&doc, ."").unwrap().as_str().unwrap(),
        "empty_key_value"
    );
    assert_eq!(
        biopsy!(&doc, ."multi\nline"."🔥")
            .unwrap()
            .as_str()
            .unwrap(),
        "chaos"
    );
}
