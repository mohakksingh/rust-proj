fn main(){
    let mut my_string=String::from("hello");
    takes_ownership(my_string);
    println!("{}", my_string);
}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}
