// enums are of 3 type -
// simple enum , option enum and result enum

enum Color{
    Red,
    Yellow,
    Blue
}

fn main(){
    let color : Color = Color::Yellow;

}

enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    fn new_circle(radius: f64) -> Self {
        return Self::Circle(radius)
    }

    fn new_rectangle(width: f64, height: f64) -> Self {
        return Self::Rectangle { width, height }
    }

    fn print(&self) {
        match self {
            Self::Circle(radius) => {
                println!("Circle with radius: {}", radius);
            }
            Self::Rectangle { width, height } => {
                println!("Rectangle with width: {} and height: {}", width, height);
            }
        }
    }
    fn area(&self){
        match self{
            Self::Circle(rad) =>{
                println!("Circle with area: {}", rad*rad);
            }

            Self::Rectangle { width, height }
        }
    }
}

fn main() {
    let circle = Shape::new_circle(5.0);
    let rectangle = Shape::new_rectangle(10.0, 20.0);

    circle.print();
    rectangle.print();
}