use std::io;

fn main() {
    loop {
        println!("Hii, here you can make your own loop!! \n");
        return_fn();
        println!("First, write your message OwO");
        let mut loop_message = String::new();
        io::stdin()
            .read_line(&mut loop_message)
            .expect("failed to read line");

        println!("Now, write how many times do you want to repeat");
        let mut loop_times = String::new();
        io::stdin()
            .read_line(&mut loop_times)
            .expect("failed to read line");
        let loop_times: u32 = match loop_times.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, type a number");
                return;
            }
        };
        println!("Okayy... do you want it with numbers on side? \n[1]. Yes \n[2]. No");
        let mut th_num = String::new();
        io::stdin()
            .read_line(&mut th_num)
            .expect("failed to read line");
        let th_num: u32 = match th_num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, type a number");
                return;
            }
        };
        println!("yay, are you readyyy???");
        return_fn();

        match th_num {
            1 => {
                make_loop_thn(loop_message, loop_times);
            }
            2 => {
                make_loop_thoutn(loop_message, loop_times);
            }
            _ => {
                println!("What?");
            }
        }
        println!("yayy cool, do you want to do it again? \n[1]. Yes \n [2]. No");
        let mut do_again = String::new();
        io::stdin()
            .read_line(&mut do_again)
            .expect("failed to read line");
        let do_again: u32 = match do_again.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, type a number");
                return;
            }
        };
        match do_again {
            1 => {
                return_fn();
                continue;
            }
            2 => {
                println!("Okayy, bye bye");
                break;
            }
            _ => {
                println!("What?");
            }
        }
    }
}

fn return_fn() {
    loop {
        let mut enter_i = String::new();
        println!("press enter to continue...");
        io::stdin()
            .read_line(&mut enter_i)
            .expect("failed to read line");

        if enter_i.trim().is_empty() {
            break;
        } else {
            println!("you typed something, please, just press enter!!");
        }
    }
}

fn make_loop_thoutn(msg: String, times: u32) {
    let mut counter = 1;
    while counter <= times {
        println!("{}", msg);
        counter += 1;
    }
}

fn make_loop_thn(msg: String, times: u32) {
    let mut counter = 1;
    while counter <= times {
        println!("{}. {}", counter, msg);
        counter += 1;
    }
}
