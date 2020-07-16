fn main(){
	print_two(["zed", "SHAW"]);
	print_two_again("zed", "SHAW");
    print_one("arg1");
    print_none();
}
fn print_two(args:[&str;2]){
	let [arg1,arg2]=args;
	println!("arg1:{}, arg2:{}",arg1,arg2);
}
fn print_two_again(arg1:&str,arg2:&str){
	println!("arg1:{}, arg2:{}",arg1,arg2);
}
fn print_one(arg1:&str){
	println!("arg1:{}",arg1);
}
fn print_none(){
	println!("I got nothing");
}
