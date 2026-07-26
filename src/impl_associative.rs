struct Rectangle{
    length:u8,
    breadth:u8
}

impl Rectangle{
    //associative functions
    fn new(new_length:u8 , new_breadth:u8)->self{ // or you can write self instead of Rectangle
        self { length:new_length, breadth: new_breadth }
    }
    //method
    fn area(&self)->u8{ // make sure to use & for burrowing of ownership
        return self.length*self.breadth
    }

    // the difference between associative function and method are the we call them
    // associative function - TypeName::function()
    //method - instance.method()

    fn change_breadth(&mut self){
        self.breadth = 1;
    }

}

fn main(){
    let r1 = Rectangle::new(10,5);

}