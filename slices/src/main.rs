fn main() {
    let something = String::from("texto sim dois");

    let s1 = primeira_palavra(&something);

    println!("{}", s1)
}

fn primeira_palavra(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
