fn main(){
    let joe:Person = Person_create("Joe Alex", 32, 64, 140);
    let frank:Person = Person_create("Frank Blank", 20, 72, 180);
    println!("Joe is at memory location {:?}", &joe);
    Person_print(joe);
    println!("Frank is at memory location {:?}", &frank);
    Person_print(frank);
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    Person_print(joe);
    frank.age += 20;
    frank.weight += 20;
    Person_print(frank);
    Person_destroy(joe);
    Person_destroy(frank);
}
Struct Person{
    name:&str,
    age:mut u32,
    height:mut u32,
    weight:mut u32,
}

impl Person{
    fn Person_create(name:&str,age:mut u32,height:mut u32,weight:mut u32)->Person{
        Person{name,age,height,weight}
    }
    fn Person_destroy(self){
        drop(self);
    }
    fn Person_print(&Person){
        println!("Name:{}",Person.name);
        println!("Age:{}",Person.age);
        println!("Height:{}",Person.height);
        println!("Weight:{}",Person.weight);
    }
}