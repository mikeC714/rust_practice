use std::io;
use std::io::Write;

#[derive(Debug)]
struct User {
    index: usize,
    name: String,
    username: String,
    email: String
}


fn get_input(prompt: &str) -> String{
    println!("{}", prompt);

    io::stdout()
        .flush()
        .unwrap();
    
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    
    return input.trim().to_string();
}


fn main(){
    let mut db: Vec<User> = Vec::new();
    loop{
        let name = get_input("Please Enter Name.");
        let email = get_input("Please Enter Email");
        let username = get_input("Please Enter A Username");

        db.push(User{
            index: db.len() + 1,
            name,
            email,
            username
        });

        println!("{:#?}", db)
    }
}

