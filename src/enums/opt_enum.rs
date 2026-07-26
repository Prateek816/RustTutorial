
// we can define enum called optionalvalue that represents either a value or the absence of value
fn main() {
    let user_id_1 = 1;
    let user_id_2 = 2;

    println!("{:?}", get_user_phone_no(&user_id_1));
    println!("{:?}", get_user_phone_no(&user_id_2));
}

fn get_user_phone_no(user_id: &i32) -> Option<i32> {
    let mob_num = 213123;

    if *user_id == 1 {
        Some(mob_num)
    } else {
        None
    }
}