// fn main() {
//     let s = String::from("hello");
// 
//     change(&s);
// }
// 
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello");
// 
//     let r1 = &mut s;
//     let r2 = &mut s;
// 
//     println!("{r1}, {r2}");
// }

// fn main() {
//     let mut s = String::from("hello");
// 
//     let r1 = &s;  // no problem
//     let r2 = &s;  // no problem
//     let r3 = &mut s;  // BIG PROBLEM
// 
//     println!("{r1}, {r2}, and {r3}");
// }

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}