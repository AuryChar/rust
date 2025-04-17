fn main() {
    // borrowing e referencias

    let s1 = String::from("sla");

    let tamanho = calcular_tamanho(&s1);

    println!("O tamanho de '{}' é {}", s1, tamanho);

    let mut s = String::from("slakk");

    modifica(&mut s);
}

fn calcular_tamanho(s: &String) -> usize {
    s.len()
}

// fn modifica(modificaa: &String) {
//      modificaa.push_str(" sla"); // nao funciona por causa do borrowing, isto é, pega emprestado e devolve, coisa q vc nao faz
// }

// temos referencias mutaveis tb

fn modifica(modificaa: &mut String) {
    modificaa.push_str(" sla");
}
