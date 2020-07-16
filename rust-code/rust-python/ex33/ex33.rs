fn main(){
    let mut i=0;
    let mut numbers:Vec<u32>=Vec::new();
    while i<6{
        println!("At the top i is {}",i);
        numbers.push(i);
        i += 1;
	    println!("numbers now:{:?}",numbers);
	    println!("At the bottom i is {}",i);
    }
    println!("The numbers:");
    for num in numbers{
        println!("{}",num);
    }
    let numbers = create_numbers(10, 2);

    println!("The number: ");

    for num in numbers{
        println!("{}",num);
    }
}	    
fn create_numbers(max:u32, step:u32)->Vec<u32>{
    let i = 0;
    let mut numbers:Vec<u32>=Vec::new();
    for i in (0..max).filter(|x| x%step==0 ){
        println!("At the top i is {}",i);
        numbers.push(i);
        println!("Numbers now:{:?} ", numbers);
        println!("At the bottom i is {}",i);
    }
    return numbers


}