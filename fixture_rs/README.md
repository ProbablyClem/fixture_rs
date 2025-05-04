# fixture_rs

## Create default fixtures for your types

This crates exposes a simple fixture Trait

```rust
pub trait Fixture {
    fn fixture() -> Self;
}
```

wich can be automaticaly derived

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
