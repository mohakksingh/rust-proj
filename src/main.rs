struct NoShape;

impl NoShape{
    fn area(&self)->u32{
        return 0;
    }
}

fn main(){
    let rect= NoShape;
    print!("{:?}",rect.area());
}