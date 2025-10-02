// fn main() {
//     let mut my = String::from("Hello World");
//     let mut my_vec: Vec<_> = vec![];

    
//     pretty(&my);
//     pretty(&my);
//     pretty(&my);
//     pretty(&my);
//     pretty(&my);
//     pretty(&my);


//     my_vec.push(&mut my);


//     loud(&mut my);
//     loud(&mut my);
//     loud(&mut my);
//     loud(&mut my);
//     loud(&mut my);

//     dbg!(my_vec);

// }

// fn pretty(print: &String) {
//     println!(">> {print} <<");
// }

// fn loud(print: &mut String) {
//     print.push_str("!!!!!!!");
//     pretty(&print);

// }

fn main() {
    let a: String = String::from("Hello world");
    let c;

    
    let b: String = String::from("I want");
    c = foo(&a, &b, 2);

    

    println!("{c}");

    let my_num = 44;
    let my_vec = vec![String::from("Hello")];

    bar(my_num, my_vec.clone());
    bar(my_num, my_vec);

}

fn foo<'a>(a: &'a String, b: &'a String, num: i32) -> &'a String {
    if num % 2 == 0 {
        a
    } else {
        b
    }
}

fn bar(a: i32, b: Vec<String>) {
    todo!()
}
/*
    T
    - read and write
    - Dropped OOS
    - can call .Drop()

    &T
    - read only
    - can be shared indefinetly
    - always points to a valid data

    &mut T
    - read/write
    - only one can exist at any given time
    - lifetimes cannot overlap
    - always points to a valid data


*/