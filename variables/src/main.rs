fn main() {
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    // consts
    const MAX_POINTS: u32 = 100_000;

    println!("{}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;

    println!("{}", y);

    let espacos = "    ";
    let espacos = espacos.len();
    println!("{}", espacos);
}
