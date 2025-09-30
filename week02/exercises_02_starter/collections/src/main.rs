use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::HashMap;

const MAX_ITER: i32 = 300000;

fn main() {

    // My expectations for:
    // Insertions
    // I think either Vec or HashMaps will be fastest 
    // as they should have a O(1) insertion
    // Linked list probably the slowest
    //
    // Removal
    // I think VecDeque removal will be fastest as it just removing the pointer,
    // unlike the other which have to rearrange after

    // Vectors
    vec_operations();

    // VecDeque
    vec_deque_operations();

    // Linked List 
    linked_list_operations();
    // Hashmap 
    hashmap_operations();

    /*

    Which collection type was the fastest for adding and removing elements?
    The fasest was VecDeque insert at 2.7ms and also VecDeque removal at 3.2ms

    Why do you think this was the case?
    Since Vec can be thought of like a double ended queue, it makes it very easy to 
    add to the end, but even easier for pop_front as we just move the front pointer,
    so there is no need for resizing and reshuffling like the others.

    Is there any significant difference between Vec and VecDeque deletion?
    Yes, Vec Removal is 3.44 seconds while VecDeque removal is 3.2ms so about 1000x difference

    If so, why? If not, why not?
    Since Vec are just like dynamic array, if we do remove the first element, it means we have to shift
    all the elements backwards one spot. Therefore since MAX_ITER was 300 000, Vec would have been shifting all 
    these elements numerous times, hence the 3.44 seconds

    When would you consider using VecDeque over Vec?
    If you want efficiency adding to the ends of the list, especially if removing at either ends
    You also want a queue

    When would you consider using LinkedList over Vec?
    If you have to split or merge lists as for LinkedList its a O(1) compute whilst vec would be O(n).

    Did the results suprise you? Why or why not?.
    Yea that firstly hashmap had a much longer time than the others, because I thought it had a O(1) for insertion and removal, 
    but I forgot to just how long the resizing and shifting would be.

    Also that insertion is much slower than removal, because of the rehashing operations.

    */


}

/// measure the insertion and removal
/// operations of a vector
fn vec_operations() {
    let mut vec = Vec::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec.push(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Vector ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec.remove(0);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

/// measure the insertion and removal
/// operations of a VecDeque
fn vec_deque_operations() {
    let mut vec_deque = VecDeque::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        vec_deque.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== VecDeque ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        vec_deque.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn linked_list_operations() {
    let mut list: LinkedList<i32> = LinkedList::new();
    
    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        list.push_back(i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Linked List ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for _ in 0..MAX_ITER {
        list.pop_front();
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);
}

fn hashmap_operations() {
    let mut hash_map = HashMap::new();

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.insert(i, i);
    }
    let time_end = std::time::Instant::now();

    println!("==== Hashmap ====");
    println!("insert: {:?}", time_end - time_start);

    let time_start = std::time::Instant::now();
    for i in 0..MAX_ITER {
        hash_map.remove(&i);
    }
    let time_end = std::time::Instant::now();

    println!("remove: {:?}", time_end - time_start);

}