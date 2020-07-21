fn main(){
    let the_count = vec![1, 2, 3, 4, 5];
    let fruits = vec!["apples", "oranges", "pears", "apricots"];
    struct Change(i32,&'static str,i32,&'static str,i32,&'static str);
    let change = Change(1, "pennies", 2, "dimes", 3, "quarters");
    for number in the_count{
        println!("This is count {}",number);
    }
    println!("I got {}",change.0);
    println!("I got {}",change.1);
    println!("I got {}",change.2);
    println!("I got {}",change.3);
    println!("I got {}",change.4);
    let mut elements=Vec::new();

    for i in 0..6{
        println!("Adding {} to the list",i);
        elements.push(i);
    }
	
	

    for i in elements{
        println!("element was: {}",i);
    }
	

	

}