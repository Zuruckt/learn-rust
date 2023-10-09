use dialoguer::{Select, Input};

fn main() {
    let items = vec!["Celsius ºC", "Farenheit ºF", "Kelvin K"];

    let convert_from = Select::new()
        .with_prompt("From which unit do you want to convert from?")
        .items(&items)
        .interact()
        .unwrap_or_default();
    
    println!("You chose to convert from {}", items[convert_from]);

    let filtered_items = items
        .iter()
        .enumerate()
        .filter(|(usize, &&_)|  *usize != convert_from)
        .map(|(_, &item)| item.to_owned())
        .collect::<Vec<String>>();

    let convert_into = Select::new()
        .with_prompt("Which unit do you wish to convert into?")
        .items(&filtered_items)
        .interact()
        .unwrap_or_default();

    println!("You chose to convert into {}", items[convert_into]);

    let original_temperature: f32 = Input::new()
        .default(10.0)
        .with_prompt("Enter the temperature:")
        .interact()
        .unwrap_or_default();

    println!("Temperature in {}: {}", items[convert_from], original_temperature)
}
