//Create our function to test if our number is even
fn is_even(n: i32) -> bool{
    n % 2 == 0
}


fn main(){

    //Create array with at least 10 int numbers
    let numbers: [i32; 10] = [2, 11, 15, 32, 20, 22, 25, 28, 30, 18];

    //Use a for loop to loop through array
    for num in numbers{
        if num % 3 == 0 && num % 5 == 0{
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0{
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0{
            println!("{}: Buzz", num);
        }
        else if is_even(num){
            println!("{} is even.", num);
        }
        else{
            println!("{} is odd.", num);
        }
    }

    //Next lets work on our while function
    let mut sum = 0;

    let mut index = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("The total sum of numbers is: {}", sum);

    //Now we are going to work on finding the largest number in the array
    let mut largest_num = numbers[0];

    let mut index = 1;

    loop {
        if numbers[index] > largest_num {
            largest_num = numbers[index];
        }

        index += 1;

        if index == numbers.len(){
            break;
        }
    }

    println!("The largest number within the array is: {}", largest_num);
}