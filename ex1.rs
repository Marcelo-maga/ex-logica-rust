/* 
Escreva um algoritmo que armazene o valor 10 em uma variável A e o valor 20 em uma variável B.
A seguir (utilizando apenas atribuições entre variáveis) troque os seus conteúdos fazendo com que o
valor que está em A passe para B e vice-versa. Ao final, escrever os valores que ficaram armazenados
nas variáveis. 
*/

fn main () {
  let mut number1 = 10;
  let mut number2 = 20;
  let mut number3 = 0;

  number3 = number1;
  number1 = number2;
  number2 = number3;

  println!("Variavel A: {} e Variavel B: {}", number1, number2)
}