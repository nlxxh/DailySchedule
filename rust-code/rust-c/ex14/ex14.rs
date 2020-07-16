use std::env;
fn main(){
    let args:Vec<String>=env::args().collect();
    print_arguments(args.len(),args);

}
fn print_arguments(argc:usize, argv:Vec<String>)
{
    for i in 1..argc{
        print_letters(&argv[i]);
    }
}
fn print_letters(x:&String){
    for i in x.chars(){
        if can_print_it(i){
            print!("{}=={}  ",i,i as i32);
        }

    }
    println!();

}
fn can_print_it(c:char)->bool{
    match c{
        'a'..='z' | ' ' | 'A'..='Z'=>true,
        _=>false,
    }
    
}
