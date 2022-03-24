mod entities;
use entities::user::User;

mod providers;
use providers::hash::HashProvider;

mod repositories;
use repositories::user::UserRepository;

mod business;
use business::user::UserBusiness;
use business::user::UserErrors;


fn handler(res: Result<(), UserErrors>) {
    match res {
        Ok(()) => println!("Created"),
        Err(err) => println!("{}", err)
    }
}

fn main() {
    let mut user_business = UserBusiness::new(
        UserRepository::new(),
        HashProvider::new("key".to_string())
    );

    let user = User::new(
        "João Victor".to_string(),
        "joao@email.com".to_string(),
        "senha123".to_string(),
    );

    let user_created = user_business.create_user(&user);
    handler(user_created);
    
    match user_business.create_user(&user) {
        Ok(()) => println!("Created"),
        Err(err) => println!("{}", err)
    }
    user_business.find();

}