use super::compare_version;

#[test]
fn newer_major_is_update() {
    // new(2.0.0) を old(1.9.9) より新しいとみなす
    assert!(compare_version("2.0.0", "1.9.9").unwrap());
}

#[test]
fn older_major_is_not_update() {
    //  new(1.9.0) を old(2.0.0) より新しいとみなさない
    assert!(!compare_version("1.9.0", "2.0.0").unwrap());
}

#[test]
fn same_version_is_not_update() {
    // new(v1.0.2) を old(1.0.2) より新しいとみなさない
    assert!(!compare_version("v1.0.2", "1.0.2").unwrap());
}

#[test]
fn invalid_version_is_err() {
    // new(1.0) を old(1.0.0) より新しいとみなさない
    assert!(compare_version("1.0", "1.0.0").is_err());
}
