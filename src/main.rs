
mod mem1;
mod burrow;
mod impl_associative;
mod enums;
mod traits;
mod vec;
mod file;
mod slices;
mod lifetime;
mod test1;
//mod design_pattern;
fn main() {
    let num = 8;
    println!("Hello, world!s");

    //Tuple
    let emp_info:(&str,u8) = ("Ramesh",50);
    let emp_name = emp_info.0;
    let emp_age = emp_info.1;

    let (emp_name,emp_age) = emp_info;

    let emp_test:(&str,u8);

    let mut r = &42;
    {
        let x = 10;
        r = &x;
    }
    println!("{}",r);

}
