use uuid::Uuid;

#[allow(dead_code)]
pub struct User {
  id:       String,
  pub name:     String,
  pub email:    String,
  password: String
}

impl User {
  pub fn new(name: String, email: String, password: String) -> Self {
    User {
      id: Uuid::new_v4().to_string(),
      name,
      email,
      password
    }
  }

  // pub fn getId(&self) -> String { self.id.clone() }
  // pub fn getEmail(&self) -> String { self.email.clone() }
  pub fn get_password(&self) -> String { self.password.clone() }
}