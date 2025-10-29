fn main() {
    // let mut a: String = String::from("hello");
    // std::thread::spawn( || {
    //     our_function(a);
    // });
    let s = String::from("hello");
    let print = move || println!("{}", s); // `s` is moved into the closure
                                           // s is no longer available here (ownership moved)
    print();

    use std::thread;
    // Example 1: move into thread and return the value from the thread
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("thread (owns v): {:?}", v);
        v // return ownership back to parent via join()
    });
    // take ownership back when the thread finishes
    let v = handle.join().unwrap();
    println!("parent got v back: {:?}", v);

    // Example 2: clone the value so parent keeps its copy
    let v = vec![4, 5, 6];
    let v_for_thread = v.clone();
    let handle = thread::spawn(move || {
        println!("thread (clone): {:?}", v_for_thread);
    });
    handle.join().unwrap();
    // original v is still available here
    println!("parent still has v: {:?}", v);

    // Example 3: share using Arc (cheap shared ownership)
    use std::sync::Arc;
    let v = Arc::new(vec![7, 8, 9]);
    let v_for_thread = Arc::clone(&v);
    let handle = thread::spawn(move || {
        println!("thread (Arc): {:?}", v_for_thread);
    });
    handle.join().unwrap();
    println!("parent Arc still available: {:?}", v);
}

// fn our_function(mut a: String) {
//     println!("{a}");
//     a.push_stre(string: "!");
//     drop(a);
// }
