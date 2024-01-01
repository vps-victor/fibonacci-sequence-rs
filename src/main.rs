use std::io;

fn main() {
    // Peça ao usuário para fornecer um número positivo
    let n: i32;

    loop {
        println!("Digite um número positivo:");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.trim().parse() {
            Ok(num) if num > 0 => {
                n = num;
                break;
            }
            Ok(_) => println!("Por favor, insira um número positivo."),
            Err(_) => println!("Por favor, insira um número válido."),
        }
    }

    // Gere os primeiros N termos da sequência de Fibonacci
    let mut fibonacci_sequence: Vec<i64> = Vec::new();
    
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n {
        fibonacci_sequence.push(a);
        let next_term = a + b;
        a = b;
        b = next_term;
    }

    // Imprima o vetor final como uma string
    let fibonacci_string: String = fibonacci_sequence.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
    println!("Sequência de Fibonacci para N = {}: {}", n, fibonacci_string);
}

