use::std::env;
use std::fs;
fn main(){
    let args:Vec<String>=env::args().collect();
    let from_file=args[1].clone();
    let to_file=args[2].clone();
    let target=args[2].clone();
    println!("Copying from {} to {}",from_file,to_file);

    let contents=fs::read_to_string(from_file).expect("error");
    println!("The input file is {} bytes long",contents.len());
    if let Ok(_)=fs::File::open(to_file){
        println!("Does the output file exists? True");
        fs::write(target,contents).unwrap();
    }
    else{
        println!("Does the output file exists? False");
    }
    
    
    
    

}