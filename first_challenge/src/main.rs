use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut results: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;
    // Caminho para o arquivo
    let path = Path::new("src/calories.txt");
    // Abrir o arquivo
    let file = File::open(&path).expect("Cannot open file");
    // Criar um buffer para ler as linhas
    let reader = io::BufReader::new(file);

    // Iterar sobre as linhas do arquivo
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            sum = 0;
        } else {
            match line.trim().parse::<u32>() {
                Ok(num) => sum += num,
                Err(_) => eprintln!("Warning: Could not parse line into number: {}", line),
            }
            results.push(sum);
        }
    }
    match results.iter().max() {
        Some(max) => println!("The maximum number is {}", max),
        None => println!("The vector is empty"),
    }
    {
        results.sort();
        results.reverse();
        let first_three: Vec<_> =results.iter().take(3).collect();
        let total: u32 = first_three.into_iter().sum();
        println!("{}", total);
    }
}



