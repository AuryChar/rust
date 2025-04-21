fn main() {
    let something = String::from("texto sim dois");

    let s1 = primeira_palavra(&something);
    // let texto = &something[0..5];
    // let sim = &something[6..9];
    // let dois = &something[10..14];

    println!("{}", s1);
}

fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
