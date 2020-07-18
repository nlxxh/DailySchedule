use std::env;
fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()!=2{
        println!("ERROR: You need one argument.");
    }
    let  mut k=0;
    for i in args[1].chars(){
        match i{
            'a' | 'A'=>{println!("{}: 'A'", k);}
            'e' | 'E'=>{println!("{}: 'E'", k);}
            'i' | 'I'=>{println!("{}: 'I'", k);}
            'o' | 'O'=>{println!("{}: 'O'", k);}
            'u' | 'U'=>{println!("{}: 'U'", k);}
            'y' | 'Y'=>{if k > 2 {
                    // it's only sometimes Y
                    println!("{}: 'Y'", k);
                }}
            _=>{println!("{}: {} is not a vowel", k, i);}
        }
        k+=1;
    }
}