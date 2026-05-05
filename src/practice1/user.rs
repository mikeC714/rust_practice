
#[derive(Debug)]
struct User {
    name: String,
    email: String
}

impl User{
    fn get_name(&self) -> &str{
    return &self.name;
    }
    fn get_email(&self) -> &str{
        return &self.email;
    }
}

fn main(){
    let user = User{name:"Mike Ock".to_string(), email:"mikeOckHurts@email.com".to_string()};
    println!("{}", user.get_name());
    println!("{}", user.get_email());
}