use learn_say_hello_pemudakoding::{
    say_hello,
    say_goodbye,
    say_hello_to_everyone,
    say_goodbye_to_everyone
};

fn main() {
    let response: String = say_hello("Stiven");
    println!("{}", response);

    let response: String = say_hello_to_everyone();
    println!("{}", response);

    let response: String = say_goodbye("Stiven");
    println!("{}", response);

    let response: String = say_goodbye_to_everyone();
    println!("{}", response);
}
