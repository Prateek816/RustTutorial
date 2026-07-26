// The result enum is a type that represnts the result of a computation that can eiterh Suceed r return a value or fail(Err) and return an error.
// It's comminly used for functions that may fail for varioes reasons

enum Result<T,E>{
    OK(T),
    Err(E),
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Y is zero".to_owned());
    } else {
        return Ok(x / y);
    }
}

fn main() {
    let result: Result<i32, String> = divide(10, 5);

    println!("{:?}", result);
}