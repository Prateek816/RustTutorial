
trait Toy{
    fn log(&self);
}

struct Robot;
struct Car;

impl Toy for Robot{
    fn log(&self){
        println!("This is a Robot")
    }
}
impl Toy for Car{
    fn log(&self){
        println!("this is a Car")
    }
}

enum ToyType{
    Robot,
    Car
}

struct Factory;
impl Factory{
    fn build_toy(toy_type:ToyType)->Box<dyn Toy> {
        match ToyType{
            ToyType::Robot => Box::new(Robot),
            ToyType::Car => Box::new(Car),
        }
    }
}

fn main(){
    let robot = Factory::build_toy(ToyType::Robot);
    let car = Factor::build_toy(ToyType::Car);
}