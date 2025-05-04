# fixture_rs

## Simple Derive Macro to generate fixtures

### Usage

You need to define a simple fixture trait in your repository

```rust
pub trait Fixture {
    fn fixture() -> Self;
}
```

And then you can implement it for primitive types, or custom structs

````rust
impl Fixture for String {
    fn fixture() -> Self {
        "string".to_string()
    }
}

impl Fixture for u32 {
    fn fixture() -> Self {
        1
    }
}

impl<T: Fixture> Fixture for Option<T> {
    fn fixture() -> Self {
        Some(T::fixture())
    }
}

impl<T: Fixture> Fixture for Vec<T> {
    fn fixture() -> Self {
        vec![T::fixture()]
    }
}

impl<T: Fixture> Fixture for Box<T> {
    fn fixture() -> Self {
        Box::new(T::fixture())
    }
}
```
Thanks to this crate, it can be automatically derived

```rust
#[derive(Fixture)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub bio: Option<String>,
}

#[derive(Fixture)]
pub struct Group {
    pub users: Vec<User>,
}
```

You can then call fixture() to use it in your tests

```rust
    #[test]
    fn test_user_fixture() {
        let user = User::fixture();
        assert_eq!(user.name, "string".to_string());
        assert_eq!(user.age, 1);
        assert_eq!(user.bio, Some("string".to_string()));
    }

    #[test]
    fn test_group_fixture() {
        let group = Group::fixture();
        assert_eq!(group.users.len(), 1);
        let user = &group.users[0];
        assert_eq!(user.name, "string".to_string());
    }

```
````

## Contributions

Contributions are welcomed in this project !

Interessting things to add includes :

- Fixture implementation for more primitives and std structs
- Fixture::fixture_builder() -> Builder method to return a builder of the struct and thus customise the object
