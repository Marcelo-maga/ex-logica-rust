/*
Escreva um algoritmo para ler o salário mensal atual de um funcionário e o percentual de reajuste.
Calcular e escrever o valor do novo salário. 
*/

use std::io;

fn main() {
  let mut salario = String::new();
  let mut perce = String::new();

  println!("Informe o salário mensal: ");

  io::stdin().read_line(&mut salario)
    .expect("Falha ao ler entrada");

  let salario: f32 = salario.trim().parse()
    .expect("Estou esperando um numero!");

  println!("Informe o percentual para reajsute: ");

  io::stdin().read_line(&mut perce)
    .expect("Falha ao ler entrada");

  let perce: f32 = perce.trim().parse() 
    .expect("Estou esperando um numero!");

  let perce = (perce/100.0)*salario;

  println!("O novo salário é de: {}", salario+perce);

}