

fn main(){
    stack_fn();//call the function that uses stack
    heap_fn();//call the function that uses heap
    update_string();//call the function that changes the size of variable at runtime
}

fn stack_fn(){
    //declare a few integers on the stack
    let a=10;
    let b=20;
    let c=a+b;
    println!("Stack function: the sum of {} and {} is {}",a,b,c);
}

fn heap_fn(){
    //create a string which is allocated on the heap
    let s1= String::from("Hello");
    let s2=String::from("World");
    let combined= format!("{} {}",s1,s2);
    println!("Heap function: Combined string is : {}",combined);

}

fn update_string(){
    //start with a base string on a heap
    let mut s=String::from("Initial string");
    println!("Before update: {}",s);
    println!("Capacity: {},lenght: {},pointer: {:p}",s.capacity(),s.len(),s.as_ptr());

    //append some text to the string
    s.push_str("add some additional text");
    println!("After update: {}",s);
}