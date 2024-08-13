use std::io::stdin;

fn main() {
    let mut conversion_type = String::new();
    let mut temperature  = String::new();

    loop {
        println!("1: Celsius para Fahrenheit");
        println!("2: Fahrenheit para Celsius");
        stdin()
        .read_line(&mut conversion_type)
        .expect("Escolha o tipo de conversão");
    
        let convert_to: i8 = match  conversion_type.trim().parse(){
            Ok(i) => i,
            _ => 0,
        };
    
        println!("Oi sou o heitor estou aqui:");
        stdin()
        .read_line(&mut temperature)
        .expect("Temperatura Inválida");
    
        let result = match convert_to {
            1 =>  ctof(temperature.trim().parse::<f64>().unwrap()),
            2 => ftoc(temperature.trim().parse::<f64>().unwrap()),
            _ => {
                println!("Escolha um caso valido");
                continue;
            }
        };
    
        println!("Resultado: {}", result);
        break;
    }
}

fn ctof(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

fn ftoc(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}