fn main() {
    // loops(avá)

    // literalmente um loop infinito
    loop {
        println!("Loop");
        break; // não mais
    }

    // loop condicional
    let mut numero = 3;

    while numero != 0 {
        println!("{}", numero);

        numero = numero - 1;
    }

    // percorrendo arrays com for

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("o valor é {}", element);
    }
}
