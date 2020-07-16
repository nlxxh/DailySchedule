fn main(){
    let ages = vec![23, 43, 12, 89, 2];
    let names=vec![String::from("Alan"),String::from("Frank"),String::from("Mary"),String::from("John"),String::from("Lisa"),];
    for i in 0..ages.len(){
        println!("{} has {} years alive.",names[i], ages[i]);
    }
    println!("---");

    let cur_age=ages.as_ptr();
    let cur_name=names.as_ptr();
    for i in 0..ages.len(){
        unsafe{
            println!("{} is {} years old.",*cur_name.offset(i as isize), *cur_age.offset(i as isize));
        }
    }
    println!("---");

    let cur_age=&ages as *const Vec<i32>;
    let cur_name=&names as *const Vec<String>;
    for i in 0..ages.len(){
        unsafe{
            println!("{} is {} years old again.",(*cur_name)[i],(*cur_age)[i]);
        }
    }
    println!("---");

    let cur_age=&ages;
    let cur_name=&names;
    for i in 0..ages.len(){
        println!("{} lived {} years so far.",cur_name[i], cur_age[i]);
    }
    println!("---");
}
