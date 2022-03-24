use crate::{
  entities::user::User
};

pub struct UserRepository {
  items: Vec<User>
}

impl UserRepository {
  pub fn new () -> Self {
    UserRepository {
      items: Vec::new(),
    }
  }

  pub fn create(&mut self, user: User) {
    self.items.push(user)
  }

  pub fn list(&mut self) {

    for i in self.items.iter() {
      println!("{}", i.email);
    }

  }

  pub fn find_by_email(&self, email: String) -> Option<&User> {
    for i in self.items.iter() {
      if i.email == email {
        return Some(i);
      }
    }

    None
  }
}