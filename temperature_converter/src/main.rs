//Lets start by declaring our main freezing point for fahrenheit and make it a constant because it will not change
const FREEZING_POINT_F: f64 = 32.0;

//Now lets create our first function where we will convert our degrees in fahrenheit to celsius
fn fahrenheit_to_celsius(f: f64) -> f64{
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

//Next lets create our next function to convert back to celsius from fahrenheit
//This here will get rid of the warning when we run the code because we do not use the function and its considered dead code
#[allow(dead_code)]
fn celsius_to_fahrenheit(c: f64) -> f64{
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}

fn main(){
    //Lets create a mutable variable for our starting temperature for fahrenheit
    let mut temperature_f: f64 = 32.0;

    //This print line will show our starting pointing at 32 degrees F
    println!("Our beginning temperature is: {}°F and it will be converted into: {:.2}°C.", temperature_f, fahrenheit_to_celsius(temperature_f));

    let mut count = 0; // Counter to track iterations

    loop {
        if count >= 5 {
            break; // Exit after 5 iterations
        }
        
        temperature_f += 1.0; // Increment temperature
        println!("{}°F is {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));

        count += 1; // Increment loop counter
    }

}