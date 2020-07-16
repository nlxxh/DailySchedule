use std::collections::HashMap;
fn main(){
    let states_name=vec![String::from("Oregen"),String::from("Florida"),String::from("California"),String::from("New York"),String::from("Michigan")];
    let states_abbr=vec![String::from("OR"),String::from("FL"),String::from("CA"),String::from("NY"),String::from("MI")];
    let states:HashMap<_,_>=states_name.iter().zip(states_abbr.iter()).collect();
    let cities_name=vec![String::from("San Francisco"),String::from("Detroit"),String::from("Jacksnoville")];
    let cities_abbr=vec![String::from("CA"),String::from("MI"),String::from("FL")];
    let mut cities:HashMap<_,_>=cities_abbr.iter().zip(cities_name.iter()).collect();
    let a=String::from("NY");
    let b=String::from("New York");
    cities.insert(&a,&b);
    let a=String::from("OR");
    let b=String::from("Portland");
    cities.insert(&a,&b);

    println!("----------");
    if let Some(x)=cities.get(&String::from("NY"))
    {println!("NY State has: {}",x);}
    if let Some(x)=cities.get(&String::from("OR"))
    {println!("OR State has: {}",x);}

    println!("----------");
    if let Some(x)=states.get(&String::from("Michigan"))
    {println!("Michigan's abbreviation is: {}",x);}
    if let Some(x)=states.get(&String::from("Florida"))
    {println!("Florida's abbreviation is: {}",x);}

    println!("----------");
    if let Some(x)=states.get(&String::from("Michigan"))
    {if let Some(y)=cities.get(x){println!("Michigan has:{}",y);}}
    if let Some(x)=states.get(&String::from("Florida"))
    {if let Some(y)=cities.get(x){println!("Florida has:{}",y);}}

    for (state,abbrev) in &states{
        println!("{} is abbreviated {}",state,abbrev);
    }

    println!("----------");
    for (abbrev,city) in &cities{
        println!("{} has the city {}",abbrev,city);
    }

	println!("----------");
    for (state,abbrev) in &states{
        print!("{} state is abbreviated {} ",state,abbrev);
        if let Some(x)=cities.get(abbrev){
            println!("and has city {}",x);
        }      
    }

    println!("----------");
    if let Some(x)=states.get(&String::from("Texas")){}
    else{println!("Sorry, no Texas.");}

    if cities.contains_key(&String::from("TX")){}
    else{println!("The city for the state 'TX' is: Does Not Exist");}







    
}