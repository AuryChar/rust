use rand::Rng;

fn main() {
    println!("wow roleta russa de sim e nao caralho q fodastico");
    let range_wow = rand::thread_rng().gen_range(1, 3);
    println!("vc vai fazer alguma coisa?");
    match range_wow {
        1 => {
            println!("sim, boa noite");
        }
        2 => {
            println!("nao, bom dia");
        }
        _ => {}
    }
}
