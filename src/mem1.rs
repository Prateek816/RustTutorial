fn main() {
    // A String stores its actual text on the heap.
    // The variable `str1` itself stores a pointer, length, and capacity
    // on the stack, which points to the heap memory.
    let str1 = String::from("Hello");

    // Ownership of the heap memory is MOVED from `str1` to `str2`.
    // No new heap memory is allocated here.
    // Only the ownership is transferred.
    let str2 = str1;

    // ERROR:
    // `str1` is no longer the owner of the heap memory.
    // After the move, Rust marks `str1` as invalid.
    // If Rust allowed `str1` to be used, then both `str1` and `str2`
    // would think they own the same heap allocation.
    //
    // At the end of `main`, both variables would try to free
    // the same heap memory, causing a DOUBLE FREE error.
    //
    // To prevent this memory safety issue, Rust invalidates `str1`
    // immediately after the move, and the compiler produces an error.
    println!("str1 = {}", str1);

    // `str2` is now the only valid owner of the String.
    // It is responsible for freeing the heap memory when it goes
    // out of scope.
    println!("str2 = {}", str2);

    let x= 2;
    let y = x;
    //this will give no error
}

// another example - 
fn main(){
    let x:String = String::from("Hello");
    process_string(x);// transfer of ownership
}

fn process_string(item:String){
    println!("some operation")
}