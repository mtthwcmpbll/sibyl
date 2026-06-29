use storage::{FsStore, Store};

#[test]
fn put_get_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    store.put("decks/x/icons/cups.png", b"hello").unwrap();
    assert_eq!(store.get("decks/x/icons/cups.png").unwrap(), b"hello");
    assert!(store.exists("decks/x/icons/cups.png"));
}

#[test]
fn missing_key_is_not_found() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    assert!(matches!(
        store.get("nope"),
        Err(storage::StoreError::NotFound(_))
    ));
}

#[test]
fn list_returns_keys_under_prefix() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    store.put("decks/x/a.png", b"1").unwrap();
    store.put("decks/x/sub/b.png", b"2").unwrap();
    store.put("decks/y/c.png", b"3").unwrap();
    let keys = store.list("decks/x").unwrap();
    assert_eq!(keys, vec!["decks/x/a.png", "decks/x/sub/b.png"]);
}

#[test]
fn delete_prefix_removes_subtree() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    store.put("drafts/s1/a", b"1").unwrap();
    store.delete_prefix("drafts/s1").unwrap();
    assert!(!store.exists("drafts/s1/a"));
}

#[test]
fn key_traversal_is_neutralized() {
    let dir = tempfile::tempdir().unwrap();
    let store = FsStore::new(dir.path());
    // ".." segments are stripped, so this stays inside the base dir.
    store.put("../escape.txt", b"x").unwrap();
    assert!(store.exists("escape.txt"));
}
