fn main() {
    // controle de fluxo eba

    // expressao if
    let numero = 3;

    if numero < 5 {
        println!("verdadeiro"); //vai dar verdadeiro pois o numero recebe o valor de 3 duh
    } else {
        println!("falso");
    }

    // else if
    let numero0 = 6;

    if numero0 % 4 == 0 {
        println!("número é divisível por 4");
    } else if numero0 % 3 == 0 {
        println!("número é divisível por 3");
    } else if numero0 % 2 == 0 {
        println!("número é divisível por 2");
    } else {
        println!("número não é divisível por 4, 3 ou 2");
    }

    // if em uma declaração let
    let condicao = true;
    let numero1 = if condicao { 5 } else { 6 }; // precisa ser o mesmo tipo
    println!("{}", numero1);
}
