use std::io;
use std::io::Write;

#[derive(Debug)]
struct User{
    index: usize,
    name: String,
    username: String,
}



fn get_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>>{
    println!("{}", prompt);

    io::stdout()
        .flush()?;

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)?;

    return Ok(input.trim().to_string());
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut db: Vec<User> = Vec::new();
    loop{
        let name = get_input("Please Input Your Name.")?;
        let username = get_input("Please Input Your Username")?; 

        if name == "stop" || username == "stop"{
            break
        };

        db.push(User{
            index: db.len() +1,
            name,
            username
        });

        println!("{:#?}", db)
    }
    Ok(())
}