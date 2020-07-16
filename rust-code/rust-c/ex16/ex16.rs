fn main(){
    let mut joe:Person = Person::Person_create("Joe Alex".into(), 32, 64, 140);
    let mut frank:Person = Person::Person_create("Frank Blank".into(), 20, 72, 180);
    println!("Joe is at memory location {:?}", &joe);
    joe.Person_print();
    println!("Frank is at memory location {:?}", &frank);
    frank.Person_print();
    joe.age += 20;
    joe.height -= 2;
    joe.weight += 40;
    joe.Person_print();
    frank.age += 20;
    frank.weight += 20;
    frank.Person_print();
    joe.Person_destroy();
    frank.Person_destroy();
}
#[derive(Debug)]
struct Person{
    name:String,
    age:u32,
    height:u32,
    weight:u32,
}

impl Person{
    fn Person_create(name:String,age:u32,height:u32,weight:u32)->Person{
        Person{name,age,height,weight}
    }
    fn Person_destroy(self){
        self.drop();
    }
    fn Person_print(&self){
        println!("Name:{}",self.name);
        println!("Age:{}",self.age);
        println!("Height:{}",self.height);
        println!("Weight:{}",self.weight);
    }
}