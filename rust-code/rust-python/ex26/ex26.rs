use::std::env;
use std::fs;
use std::io::stdin;
fn main(){
    let mut age=String::new();
    let mut height=String::new();
    let mut weight=String::new();
    println!("How old are you?   ");
    std::io::stdin().read_line(&mut age).unwrap();
    println!("How tall are you?   ");
    std::io::stdin().read_line(&mut height).unwrap();
    println!("How much do you weigh?   ");
    std::io::stdin().read_line(&mut weight).unwrap();
    println!("So, you're {} old, {} tall and {} heavy.",age.trim(),height.trim(),weight.trim());
    let args:Vec<String>=env::args().collect();
    let filename=args[1].clone();
    println!("Here's your file {}:",filename);
    let contents=fs::read_to_string(filename).expect("error");
    println!("{}",contents);
    println!("Type the filename again:");
    let mut file_again = String::new();
    std::io::stdin().read_line(&mut file_again).unwrap();
    let contents2=fs::read_to_string(file_again.trim()).expect("error");
    println!("{}",contents2);
    println!("Let's practice everything.");
    println!("You\'d need to know \'bout escapes with \\ that do \n newlines and \t tabs.");
    let poem = "
    \tThe lovely world
    with logic so firmly planted
    cannot discern \n the needs of love
    nor comprehend passion from intuition
    and requires an explanation
    \n\t\twhere there is none.
";

    println!("--------------");
    println!("{}",poem);
    println!("--------------");
    let five = 10 - 2 + 3 - 6;
    println!("This should be five: {}",five);
    let start_point:u32 = 10000;
    let (beans, jars, crates)= secret_formula(start_point);
    println!("We'd have {} beans, {} jars, and {} crates.",beans,jars,crates);
    let start_point = start_point / 10;

    println!("We can also do that this way:");
    let formula =secret_formula(start_point);
    println!("We'd have {:?} beans, {:?} jars, and {:?} crates.",formula.0,formula.1,formula.2);
    let people = 20;
    let cats = 30;
    let dogs = 15;
    if people < cats{
        
    }

  

}
fn secret_formula(started:u32)->(u32,u32,u32){
	let jelly_beans = started * 500;
	let jars = jelly_beans / 1000;
	let crates = jars / 100;
	(jelly_beans, jars, crates)
}