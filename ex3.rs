// Escreva um algoritmo para ler as dimensões de um retângulo (base e altura), 
// calcular e escrever a área do retângulo. 

use std::io;


fn main() {
  let mut base = String::new();
  let mut altura = String::new();

  println!("Informe o valor da Base: ");

  io::stdin().read_line(&mut base).expect("Falha ao ler entrada");

  let base: u32 = base.trim().parse()
  .expect("Estou esperando um numero!");

  println!("Informe o valor da Altura: ");

  io::stdin().read_line(&mut altura).expect("Falha ao ler entrada");

  let altura: u32 = altura.trim().parse()
  .expect("Estou esperando um numero!");

  let area = base * altura;

  println!("A área do quadrado é: {}", area);

}