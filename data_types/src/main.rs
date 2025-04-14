fn main() {
    // data types yupie
    let number1: i32 = 23;
    println!("numero inteiro {}", number1);
    let float2: f64 = 23.000;
    println!("numero flutuante {:.0}", float2);

    // Tupla: elemento composto q pode ter varios tipos de dados juntos, foda
    let tupla: (i32, f64, u8) = (512, 3.0, 244);
    println!("tupla: {:?}", tupla);
    // desestruturação:
    let tup = (512, 3.0, 244);
    let (sla1, sla2, sla3) = tup;
    println!("tupla(desestruturada): {} {:.1} {}", sla1, sla2, sla3);
    // ou:
    let num1 = tup.0;
    let num2 = tup.1;
    let num3 = tup.2;
    println!(
        "tupla(desestruturada de outra forma): {} {:.1} {}",
        num1, num2, num3
    );

    // arrays(matrizes): mesma porra da tupla mas tem q ser tudo do mesmo tipo; nao pode ser alterado, caso precise alterar, usar vetores.
    let array_qualquer = [1, 2, 3];
    // acessando:
    let a1 = array_qualquer[0]; // primeiro indice e por ai vai
    println!("{}", a1);
}
