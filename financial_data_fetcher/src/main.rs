use serde::Deserialize;
use std::{fs::File, io::Write, thread, time::Duration};
use ureq;

//Define our pricing
trait Pricing{
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

//Below will be all the structs we are creating for this project

#[derive(Debug, Deserialize)]
struct Bitcoin{
    price: f64,
}

#[derive(Debug, Deserialize)]
struct Ethereum{
    price: f64,
}

#[derive(Debug, Deserialize)]
struct SP500{
    price: f64,
}

//Next we are going to begin implementing the traits for our structs

//Implementing Bitcoin
impl Pricing for Bitcoin{
    fn fetch_price(&mut self){
        let response: serde_json::Value = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
            .call()
            .unwrap()
            .into_json()
            .unwrap();

            self.price = response["bitcoin"]["usd"].as_f64().unwrap_or(0.0);
        println!("Bitcoin: ${}", self.price);

    }

    fn save_to_file(&self){
        let mut file = File::create("Bitcoin.txt").unwrap();
        writeln!(file, "Bitcoin price: ${}", self.price).unwrap();
    }
}


//Next we implement Etherium
impl Pricing for Ethereum{
    fn fetch_price(&mut self){
        let response: serde_json::Value = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd")
            .call()
            .unwrap()
            .into_json()
            .unwrap();

        self.price = response["ethereum"]["usd"].as_f64().unwrap_or(0.0);
        println!("Etherium: ${}", self.price);

    }

    fn save_to_file(&self){
        let mut file = File::create("Etherium.txt").unwrap();
        writeln!(file, "Etherium price: ${}", self.price).unwrap();
    }
}

//Lastly we implement the SP500, but because it is hard to find access to free apis we are only using dummy values
impl Pricing for SP500{
    fn fetch_price(&mut self){
        self.price = 5000.50;
        println!("SP500: ${}", self.price);
    }

    fn save_to_file(&self){
        let mut file = File::create("SP500.txt").unwrap();
        writeln!(file, "SP500 Value: ${}", self.price).unwrap();
    }

}

//Main fn
fn main(){

    //Create our variables for each of the three structs
    let mut bitcoin = Bitcoin { price: 0.0};
    let mut etherium = Ethereum { price: 0.0};
    let mut sp500 = SP500 { price: 0.0};


    loop{
        //
        //Now we implement the loop to show our prices every 10 seconds
        println!("Beginning of financial data fetcher\n");

        //Call the functions we implemented earlier for all three twice

        //Bitcoin
        bitcoin.fetch_price();
        bitcoin.save_to_file();

        //Etherium
        etherium.fetch_price();
        etherium.save_to_file();

        //SP500
        sp500.fetch_price();
        sp500.save_to_file();

        println!("\nIteration is done! Waiting 10 seconds.\n");
    
        //To wait for 10 seconds
        thread::sleep(Duration::from_secs(10));
    }

}
