fn main(){
    let ten_things = String::from("Apples Oranges Crows Telephone Light Sugar");
    println!("Wait there are not 10 things in that list. Let's fix that.");
    let mut stuff : Vec<_>= ten_things.split(' ').collect();
    let mut more_stuff = vec!["Day", "Night", "Song", "Frisbee","Corn", "Banana", "Girl", "Boy"];
    while stuff.len() != 10{
        let next_one = more_stuff.pop();
        if let Some(thing)=next_one{
            println!("Adding:{}", thing);
            stuff.push(thing);
            println!("There are {} items now.",stuff.len());
        }
        
    }
    println!("There we go:{:?} ", stuff);
    println!("Let's do some things with stuff.");
	
    println!("{:?}",stuff[3..5].join("#"));
    println!("{}",stuff[1]);
    println!("{}",stuff[9]);
    if let Some(thing)=stuff.pop(){
        println!("{}",thing);
    }
    println!("{}",stuff.join(" "));
	
	
	

}