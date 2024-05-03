struct Rect{
    width: u32,
    height:u32,
}

impl Rect{
    fn area(&self)-> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self)->u32{
        return 2 *(self.width*self.height);
    }
}

fn main(){
    let rect=Rect{
        width:30,
        height:50,
    };
    print!("The area of rectangle is : {}",rect.area());
    print!("The perimeter of rectangle is : {}", rect.perimeter());
}