fn main(){
    let greeting: String=String::from("hello world");
    println!("{}",greeting);

    let char1 = greeting.chars().nth(0);
    match char1{
        Some(c) => println!("{}",c),
        None => println!("No character found"),
    }
    print!("{}",char1.unwrap());
}