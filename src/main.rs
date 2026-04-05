use std::io;
use rand::Rng;
// use std::cmp::Ordering;

// secret number for the gen num
// input user input convert to number
// return statement based on the comparison on the user number and random number


fn gen_num(low: u32, high: u32) -> (Vec<String>, Vec<String>){
    let mut vec: Vec<String> = Vec::new();
    loop{

        let mut rng = rand::thread_rng();
        let numbers: u32 = rng.gen_range(low..=high);
        let six_digit_codes= format!("{:06}", numbers);

        vec.push(six_digit_codes);

        if vec.len() == 400 {
            let mut batches: Vec<Vec<String>> = vec
                .chunks(200)
                .map(|chunk| chunk.to_vec())
                .collect();
            
            let batch1 = batches.pop().unwrap();
            let batch2 = batches.pop().unwrap();

            return(batch1, batch2);
        }
    }
}


fn main(){
    println!("Please Input the Low Range Value");
    println!("Please Input the High Range Value");
    loop{
        let mut low = String::new();
        let mut high = String::new();

         io::stdin()
            .read_line(&mut low)
            .expect("Failed to read low value");    
        io::stdin()
            .read_line(&mut high)
            .expect("Failed to read high value");
            
        let guessed_high: u32 = match high.trim().parse(){
            Ok(hi) => hi,
            Err(_) => continue
        };
        let guessed_low: u32 = match low.trim().parse(){
            Ok(low) => low,
            Err(_) => continue
        };

        let (batch1, batch2) = gen_num(guessed_low, guessed_high);
        println!("Batch1: {:#?}", batch1);
        println!("Batch2: {:#?}", batch2);
        break;

    }
}



            // for chunk in vec.chunks(100){
            //     println!("{:#?}", chunk);
            // }


// fn guessing_game(){
//     loop{
//         let secret_num = rand::thread_rng().gen_range(1..=100);
//         println!("Guess a number!");
//         println!("Please input a number!");
       
//         let mut guess = String::new();

//        io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");

//         let guessed_num: u32 = match guess.trim().parse(){
//             Ok(num) => num,
//             Err(_) => continue
//         };
        
//         match guessed_num.cmp(&secret_num){
//             Ordering::Less | Ordering::Greater => println!("Good attempt the answer was {secret_num}, not {guessed_num}"),
//             Ordering::Equal => {
//                 println!("Congrats you're correct the number was {guessed_num}");
//                 break;
//             }
//         } 
//     }
// }

// fn fizz_buzz(num: i32){
//     if num % 3 == 0 && num % 5 == 0 {
//         println!("FizzBuzz")
//     } else if num % 3 == 0 {
//         println!("fizz")
//     }else if num % 5 == 0 {
//         println!("buzz")
//     }else{
//         println!("{num}")
//     }
// }

// Attempts need to be conformed into 1 Single attempt.
// WHAT NEEDS TO BE MONITORED :{
//  Code patterns, code patterns tried, the possiblility of a pattern 
// GOAL: {
//  has 6 digit password, numbers length is 7,  
//}
// }

