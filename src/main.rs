enum Shape{
    Circle(f64),
    Square(f64),
    Rectange(f64,f64),
}

fn calculate_area(shape:Shape)->f64{
    //calculate the area of the shape
    let ans= match shape{
        Shape::Circle(radius)=>3.14*radius*radius,
        Shape::Rectange(width,height)=>{
            width*height
        },
        Shape::Square(side)=>side*side
    };
    return ans;
}

fn main(){
    let circle=Shape::Circle(5.0);
    let square=Shape::Square(10.0);
    let rect=Shape::Rectange(21.23, 4.4);
    
    let area=calculate_area(circle);
    print!("Area of circle is {}\n",area);
}