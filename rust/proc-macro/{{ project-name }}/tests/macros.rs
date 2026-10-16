use std::collections::HashMap;

use {{ project-name | replace: "-", "_" }}::{Getter, duration, hashmap};

#[derive(Getter)]
struct User {
    #[getter(copy)]
    id: u64,
    name: String,
    #[getter(skip)]
    #[allow(dead_code)]
    password: String,
    #[getter(copy, name = "user_age")]
    age: u32,
}

#[derive(Getter)]
#[getter(prefix = "get_")]
struct Point {
    #[getter(copy)]
    x: i32,
    #[getter(copy)]
    y: i32,
    label: String,
}

#[test]
fn derive_getter_works() {
    let user = User {
        id: 7,
        name: "alice".to_owned(),
        password: "secret".to_owned(),
        age: 18,
    };

    assert_eq!(user.id(), 7);
    assert_eq!(user.name(), "alice");
    assert_eq!(user.user_age(), 18);

    let point = Point {
        x: 1,
        y: 2,
        label: "origin".to_owned(),
    };
    assert_eq!(point.get_x(), 1);
    assert_eq!(point.get_y(), 2);
    assert_eq!(point.get_label(), "origin");
}

#[duration]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[duration(us)]
fn unit() -> &'static str {
    "ok"
}

#[test]
fn duration_works() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(unit(), "ok");
}

#[test]
fn hashmap_macro_works() {
    let map: HashMap<String, i32> = hashmap! {
        "a" => 1,
        "b" => 2,
    };
    assert_eq!(map.len(), 2);
    assert_eq!(map["a"], 1);
    assert_eq!(map["b"], 2);

    let empty: HashMap<String, i32> = hashmap! {};
    assert!(empty.is_empty());
}
