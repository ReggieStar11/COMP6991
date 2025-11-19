use std::ptr::null_mut as null;


unsafe fn list_delete_first(list: *mut List<i32>) -> *mut List<i32> {
  
    if list == null() {
        return null();
    }
    let next = unsafe { (*list).next };


    unsafe {
        free_node(list);
    }
    return next;
}

/////////////////////////////////////////
// DO NOT MODIFY CODE BELOW THIS POINT //
/////////////////////////////////////////

fn main() {
    let list = my_linked_list();

    unsafe {
        print_list(list);
        let list = list_delete_first(list);
        print_list(list);
        free_list(list);
    }
}


struct List<T> {
    value: T,
    next: *mut List<T>,
}

fn my_linked_list() -> *mut List<i32> {
    let nums = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse::<i32>()
                .expect(&format!("Failed to parse {arg} as i32"))
        })
        .collect::<Vec<_>>();

    let mut curr_node = None;
    for value in nums.into_iter().rev() {
        curr_node = Some(Box::into_raw(Box::new(List {
            value,
            next: curr_node.unwrap_or_else(null),
        })));
    }
    curr_node.unwrap_or_else(null)
}


unsafe fn print_list(list: *mut List<i32>) {
    println!("=== PRINTING LIST ===");
    let mut curr = list;
    while curr != null() {
        println!("{}", unsafe { (*curr).value });
        curr = unsafe { (*curr).next };
    }
    println!("=====================");
}


unsafe fn free_node(node: *mut List<i32>) {
    drop(unsafe { Box::from_raw(node) });
}


unsafe fn free_list(list: *mut List<i32>) {
    let mut curr = list;
    while curr != null() {
        let to_free = curr;
        curr = unsafe { (*curr).next };

        unsafe { free_node(to_free) };
    }
}
