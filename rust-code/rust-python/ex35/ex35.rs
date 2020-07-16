fn main(){
  start();

}
fn gold_room(){
    println!("This room is full of gold. How much do you take?");
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).expect("error");
    let choice:i32 = input.trim().parse().expect("Man, learn to type a number.Good job!");
	if choice < 50{
		println!("Nice, you're not greedy, you win!");
		}
	else{
		dead("You greedy bastard!");
    }
}
fn dead(a:&str){
    println!("{} Good job!",a);
}
fn bear_room(){
	println!("There is a bear here.");
	println!("The bear has a bunch of honey.");
	println!("The fat bear is in front of another door.");
	println!("How are you going to move the bear?");
	static bear_moved :bool= true;
	loop{
        println!("{}", bear_moved);
		let mut input:String=String::new();
        std::io::stdin().read_line(&mut input).expect("error");
        let input=input.trim();
		if input.eq("take honey"){
			dead("The bear looks at you then slaps your face.");
            break;
        }
		else if input.eq("taunt bear".into()) && bear_moved{
			println!("The bear gets pissed off and chews your leg off.");
            static bear_moved :bool= false;
            break;
        }
        else if input.eq("taunt bear".into()) && !bear_moved{
			dead("The bear gets pissed .");
            break;
        }
		else if input.eq("open door".into()) && bear_moved{
				gold_room();
                break;
        }
		else{
            println!("I got no idea what that means.");
            break;
            }
    }
}
fn cthulhu_room(){
	println!("Here you see the great evil Cthulhu.");
	println!("He, it, whatever stares at you and you go insane.");
	println!("Do you flee for your life or eat your head?");
	let mut input:String=String::new();
    std::io::stdin().read_line(&mut input).expect("error");
	if input.contains("flee"){start();}	
	else if input.contains("head"){
		dead("Well that was tasty!");
    }
	else{
		cthulhu_room()
    }
}
fn start(){
    println!("You are in a dark room.");
	println!("There is a door to your right and left.");
	println!("Which one do you take?");
	let mut input:String=String::new();
    std::io::stdin().read_line(&mut input).expect("error");
	if input.trim() == "left"{
		bear_room();
    }
	else if input.trim() == "right"{
		cthulhu_room();
        }
	else{
		dead("You stumble around the room until you starve.");}
}