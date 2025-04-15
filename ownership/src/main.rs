// fn main() {
//     // ownership, string e etc

//     // string nao literal
//     let mut s = String::from("texto");

//     s.push_str(" sla");
//     println!("{}", s);

//     // memória e alocação

//     let s1 = String::from("sla");
//     let s2 = s1.clone();
//     println!("s1: {} e s2: {}", s1, s2);

//     let stringqualquer = String::from("texto");

//     toma_posse(stringqualquer);

//     // println!("{}", stringqualquer); nao funciona mais pois a função "toma_posse" pegou sua posse(ownership)
//     // para isso nao acontecer basta usar .clone()
//     let numeroqualquer = 3;

//     faz_uma_copia(numeroqualquer);
//     println!("{}", numeroqualquer); // funciona pois inteiros são Copy
// }

// fn toma_posse(uma_string: String) {
//     println!("{}", uma_string);
// }

// fn faz_uma_copia(um_inteiro: i32) {
//     println!("{}", um_inteiro);
// }

// retorno de valores em escopo
fn main() {
    let s1 = entrega_valor();

    let s2 = String::from("text2");

    let s3 = pega_e_entrega_valor(s2);

    println!("{} {}", s3, s1);
}

fn entrega_valor() -> String {
    let uma_string = String::from("Text");
    uma_string
}

fn pega_e_entrega_valor(uma_string: String) -> String {
    uma_string
}
