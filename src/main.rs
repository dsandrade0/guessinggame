use std::io;
use std::cmp::Ordering;
use rand::{Rng};

fn main() {
    header();

    let secret = rand::thread_rng().gen_range(1..=100);
    let mut i: u8 = 1;
    let mut trying = String::new();

    while true {
        trying = String::new();

        println!("Please input your {i}ª guess.");
        io::stdin()
            .read_line(&mut trying)
            .expect("Something wrong isn't right");

        let x: i16 = trying.trim_end().parse().expect("Please, type a number!");

        let exit: i16 = -1;

        if exit == x {
            println!("\n \n Bye bye");
            break;
        }


        match x.cmp(&secret) {
            Ordering::Less => println!("Small than!! \n"),
            Ordering::Greater => println!("Big than!! \n"),
            Ordering::Equal => {
                println!("\n \n Correct!!! Congratulations!!  🎉\n");
                break;
            }
        }
        i += 1;
    }

}

fn header() {
    println!("");
    println!("");
    println!("#######                          #######");
    println!("####### Welcome to Guessing Game #######");
    println!("#######                          #######");
    println!("#######      Enter -1 to exit    #######");
    println!("#######                          ####### \n\n");
}
