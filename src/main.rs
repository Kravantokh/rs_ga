use decimal::d128 as vec_float;

#[derive(Debug)]
pub struct MultiVector2{
    s: vec_float,
    x: vec_float,
    y: vec_float,
    b: vec_float,
}

impl MultiVector2{

    fn add(&self, v : MultiVector2) -> MultiVector2{
        MultiVector2{
            s: v.s + self.s,
            x: v.x + self.x,
            y: v.y + self.y,
            b: v.b + self.b,
        }
    }

    fn mult(&self, v: MultiVector2) -> MultiVector2{
        MultiVector2{
            s: self.s*v.s + self.x*v.x + self.y*v.y - self.b*v.b,
            x: self.s*v.x + self.x*v.s - self.y*v.b + self.b*v.y,
            y: self.s*v.y + self.x*v.b + self.y*v.s - self.b*v.x, 
            b: self.s*v.b + self.x*v.y - self.y*v.x + self.b*v.s,
        }
    }
}


fn main() {
    let v1 = MultiVector2{s: 0.0, x: 1.0, y: 0.0, b: 0.0};
    let v2 = MultiVector2{s: 0.0, x: 0.0, y: 1.0, b: 0.0};
    println!("{:?}", v1);
    println!("{:?}", v2);
    let v3 = v1.mult(v2);
    println!("{:?}", v3);
}
