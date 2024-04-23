struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn from_email(email: String) -> Option<Self> {
        // Logic to retrieve user details from the email address
        let username = email.split('@').next().unwrap_or_default().to_string();
        
        // If user details are found, create and return a new User instance
        Some(Self {
            username,
            email,
            uri: String::from("https://example.com"),
            active: true,
        })
    }
}

fn main() {
    let mut new_user = User::from_email("mlsween55@gmail.com".to_string()).unwrap();
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
}
