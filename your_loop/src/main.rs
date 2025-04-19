use std::io;

fn main() {
    loop {
        println!("Hii, here you can make your own loop!! \n");
        return_fn();
        println!("First, write your message OwO");
        let mut loop_message = String::new();
        read_auto(&mut loop_message);

        println!("Now, write how many times do you want to repeat");
        let mut loop_times = String::new();
        read_auto(&mut loop_times);
        let loop_times: u32 = match loop_times.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please, type a number");
                return;
            }
        };
        println!("Okayy... do you want it with numbers on side? \n[Y]. Yes \n[N]. No");
        let mut th_num = String::new();
        read_auto(&mut th_num);
        let th_num = th_num.trim().to_uppercase();
        let value: bool;
        match th_num.as_str() {
            "Y" => {
                value = true;
                make_loop(loop_message, loop_times, value);
            }
            "N" => {
                value = false;
                make_loop(loop_message, loop_times, value);
            }
            _ => {
                println!("What?");
            }
        }
        println!("yay, are you readyyy???");
        return_fn();

        println!("yayy cool, do you want to do it again? \n[Y]. Yes \n[N]. No");
        let mut do_again = String::new();
        read_auto(&mut do_again);
        let do_again = do_again.trim().to_uppercase();
        match do_again.as_str() {
            "Y" => {
                return_fn();
                continue;
            }
            "N" => {
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

fn make_loop(msg: String, times: u32, bool_f_num: bool) {
    let mut counter = 1;
    if bool_f_num {
        while counter <= times {
            println!("{}. {}", counter, msg);
            counter += 1;
        }
    } else {
        while counter <= times {
            println!("{}", msg);
            counter += 1;
        }
    }
}

fn read_auto(msg: &mut String) {
    io::stdin().read_line(msg).expect("failed to read line");
}
