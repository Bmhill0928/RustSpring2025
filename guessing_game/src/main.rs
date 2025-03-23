//Checking our guess function
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main(){

    //secret num
    let secret = 27;

    //initial guess
    let mut guess = 20;

    let mut num_attempts = 0;

    loop{

        num_attempts += 1;

        let final_result = check_guess(guess, secret);

        if final_result == 0 {
            
            println!("Correct! The secret number is {}.", secret);
            break;

        } else if final_result == 1 {

            println!("{} is too high please try again.", guess);
            guess -= 1;

        } else {

            println!("{} is too low please try again.", guess);
            guess += 1;

        }

    }

    println!("It took {} guesses to find the correct number.", num_attempts)

}