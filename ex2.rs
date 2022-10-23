// Escreva um algoritmo para ler um valor (do teclado) e escrever (na tela) o seu antecessor. 

use std::io;

fn main() {
  println!("Informe um valor: ");

  let mut valor = String::new();

  io::stdin().read_line(&mut valor)
  .expect("Falha ao ler entrada");

  let valor: u32 = valor.trim().parse()
   .expect("Estou esperando um numero");

  println!("{}", valor-1)
}