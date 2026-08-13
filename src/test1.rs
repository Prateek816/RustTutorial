pub fn f(x:i32)->i32{
    x+x
}

pub fn g(x:i32)->i32{
    x.pow(4)
}

fn main(){
    let mut local = f;
    local = g;
    /*
 let mut local = f;
   |                     - expected due to this value
11 |     local = g;
   |             ^ expected fn item, found a different fn item
   |
   = note: expected fn item `fn(_) -> _ {f}`
              found fn item `fn(_) -> _ {g}`
   = note: different fn items have unique types, even if their signatures are the same
   = help: consider casting both fn items to fn pointers using `as fn(i32) -> i32`
*/

   let mut A :fn(_)->_=f;
   A = g;
   // no issue
}
