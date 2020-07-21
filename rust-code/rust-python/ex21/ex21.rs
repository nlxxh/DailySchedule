fn main(){
    println!("Let's do some math with just functions:");
    let age = add(30, 5);
    let height = subtract(78, 4);
    let weight = multiply(90, 2);
    let iq = divide(100, 2);
    println!("Age: {}, Height: {}, Weight:{}, IQ: {}",age,height,weight,iq);
    println!("Here is a puzzle.");
    let what = add(age, subtract(height, multiply(weight, divide(iq, 2))));
    println!("That becomes: {} Can you do it by hand?",what);
    let num1 = 40;
    let num2 = 50;
    let num3 = 50;

    println!("{}",isequal(num1, num2));
    println!("{}",isequal(num2, num3));
    println!("Here is a new puzzle.");
    let uplen = 50;
    let downlen = 100;
    let height = 80;
    let what_again = divide(multiply(height, add(uplen, downlen)), 2);

    println!("That become: {} Bazinga!",what_again);



}
fn add(a:i32,b:i32)->i32{
    println!("ADDING {} + {}",a,b);
    a+b
}
fn subtract(a:i32,b:i32)->i32{
    println!("SUBTRACTING {} - {}",a,b);
    a-b
}
fn multiply(a:i32,b:i32)->i32{
    println!("MULTIPLYING {} * {}",a,b);
    a*b
}
fn divide(a:i32,b:i32)->i32{
    println!("DIVIDING {} / {}",a,b);
    a/b
}
fn isequal(a:i32, b:i32)->bool{
    println!("Is {} equal to {}? ",a,b);
    a == b
}