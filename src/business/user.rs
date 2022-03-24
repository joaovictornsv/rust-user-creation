use crate::{
  providers::hash::HashProvider,
  entities::user::User,
  repositories::user::UserRepository,
};

use std::fmt;

pub struct UserBusiness {
  user_repository: UserRepository,
  hash_provider: HashProvider
}

pub enum UserErrors {
  UserAlreadyExists
}

impl fmt::Display for UserErrors {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let msg = match self {
      UserErrors::UserAlreadyExists => "user already exists",
    };
    write!(f, "{}", msg)
}
}

impl UserBusiness {
  pub fn new(_user_respository: UserRepository, _hash_provider: HashProvider) -> Self {
    UserBusiness {
      user_repository: _user_respository,
      hash_provider: _hash_provider
    }
  }

  pub fn create_user(&mut self, user: &User) -> Result<(), UserErrors> {
    let password_hash = self.hash_provider.hash(user.get_password().as_bytes());

    let user_exists = self.find_by_email(user.email.clone());

    if !user_exists.is_none() { return Err(UserErrors::UserAlreadyExists) }

    let user = User::new(user.name.clone(), user.email.clone(), password_hash);
    self.user_repository.create(user);
    Ok(())
  }

  pub fn find_by_email(&self, email: String) -> Option<&User> {
    let user = self.user_repository.find_by_email(email);

    user
  }

  pub fn find(&mut self) {
    self.user_repository.list();
  }
}
