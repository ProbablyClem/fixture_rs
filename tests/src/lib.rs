use fixture::Fixture;

// Exemple de struct pour test
#[derive(Fixture, Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub bio: Option<String>,
}

#[derive(Fixture, Debug, PartialEq)]
pub struct Group {
    pub users: Vec<User>,
}

// Tests unitaires
#[cfg(test)]
mod tests {
    use super::*;

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
}
