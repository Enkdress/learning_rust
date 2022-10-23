fn main() {
    let dollar_cop = 4900.0;
    let dollar_eur = 1.01;

    println!("Welcome to the rust exchage currency app");
    println!("----------------------------------------\n");

    println!("Select one of these currencies: EUR or COP");
    let mut exchange_currency = String::new();
    std::io::stdin().read_line(&mut exchange_currency).unwrap();
    exchange_currency = exchange_currency.trim().to_string();

    println!("How many {} you want to convert to USD", exchange_currency);
    let mut currency_value = String::new();
    std::io::stdin().read_line(&mut currency_value).unwrap();
    currency_value = currency_value.trim().to_string();
    let currency_value_number: f64 = currency_value.as_str().parse().unwrap();

    // Check for the allowed currencies
    if exchange_currency == "EUR" {
        operation(dollar_eur, currency_value_number, exchange_currency);
    } else if exchange_currency == "COP" {
        operation(dollar_cop, currency_value_number, exchange_currency);
    } else {
        println!("Ohhh sorry. Currency not allowed yet.");
    }
}

fn operation(dollar_val: f64, currency_val: f64, selected_currency: String) {
    let operation = currency_val / dollar_val;
    println!(
        "Result:\n{} {} are {:.2} USD",
        currency_val, selected_currency, operation
    );
}
