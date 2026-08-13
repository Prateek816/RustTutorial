fn main(){

    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(),string2.as_str());
    println!("The longest String is {}",result)
}

fn longest<'a>(x:&'a str,y:&'a str)->&'a str{ // here the smallest lifetime will be returned as the reference
    if x.len()>y.len(){
        x
    }
    else {
        y
    }
}