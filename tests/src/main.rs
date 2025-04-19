fn main() {
    let phrase: String = String::from("alguma coisa ai só pra ter espaços");
    let quantity = count_words(&phrase);
    println!("Frase: {} \nquantidade de espaços: {}", phrase, quantity);
}

fn count_words(msg: &str) -> usize {
    msg.matches(" ").count()
}