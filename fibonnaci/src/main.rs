use std::io::stdin;

fn main() {
    let mut temp = String::new();
    println!("Insira a quantidade de digitos que deseja:");
    stdin()
    .read_line(&mut temp)
    .expect("Quantidade inválida.");
    
    match temp.trim().parse() {
        Ok(num) => fib(num),
        _ => println!("Quantidade inválida.")
    };
}

fn fib(n: i64) {
    let mut first: i64 = 1;
    let mut second: i64 = 0;
    let mut i: i64 = 0;

    while i < n{
        match i {
            0 => print!("{}", second),
            _ => print!(" -> {}", second)
        }
        let temp  = first + second;
        second = first;
        first = temp;

        i += 1;
    };
}
