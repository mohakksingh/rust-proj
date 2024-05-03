fn main(){
    let mut s1 =String::from("Hello,rust");
    update_string(&mut s1);
    print!("{}",s1);
}

fn update_string(s:&mut String){
    s.push_str("World")
}