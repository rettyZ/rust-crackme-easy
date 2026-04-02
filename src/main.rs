use std::io;

fn check(username: &str, password: &str) -> bool {
    let hardcoded_username = "crackers";
    let hardcoded_password = "passhunt";

    if username == hardcoded_username && password == hardcoded_password
    {
        println!("\nCorrect!!!");
        return true;
    }
    else {
        println!("\nWRONG!!!");
        return false;
    }
}

fn main() {
    loop {
        let mut username = String::new();
        let mut password = String::new();

        println!("Please enter your username:");
        io::stdin().read_line(&mut username).expect("Failed to read line");

        println!("Please enter your password:");
        io::stdin().read_line(&mut password).expect("Failed to read line");

        if check(&username.trim(), &password.trim()) == true {
            break;
        }
        print!("\n");
    }
}
