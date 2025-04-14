fn main() {
    // funcoes
    println!("sim");
    outra_funcao_ava();
    dnv_com_parametros(100 /*, 20.10 */);

    // declaração e expressões
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    let _cinco = cinco();

    let sla1 = soma_um(1);
    println!("{}", sla1);

    println!("o valor de y é {}", y);
}

// isso definitivamente eh uma funçao
fn outra_funcao_ava() {
    println!("avá");
}

fn dnv_com_parametros(x: i32 /*y: f64  //outro parametro caso preciso */
) {
    println!(
        "teste do parametro(deve retornar oq colocar na hora de chamr essa bosta): {}",
        x
    );
    // println!("parametro 2: {}", y);
}

// funcoes com valor de retorno
fn cinco() -> i32 {
    5
}

fn soma_um(sla: i32) -> i32 {
    sla + 1
}
