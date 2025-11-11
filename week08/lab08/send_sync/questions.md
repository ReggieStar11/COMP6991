1) I saw someone's code fail to compile because they 
were trying to send non-thread-safe data across threads. 
How does the Rust language allow for static (i.e. at compile time)
guarantees that specific data can be sent/shared acrosss threads?

Rust uses its ownership model and traits like Send and Sync to guarantee at compile time that data can be safely transferred or shared across threads, preventing data races.

2) Do you have to then implement the Send and Sync traits for 
every piece of data (i.e. a struct) you want to share and send across threads?

You don’t typically need to manually implement Send and Sync for most types, as Rust automatically derives them based on the type's fields, unless you need custom behavior.

3) What types in the course have I seen that aren't Send? Give one example, 
and explain why that type isn't Send 

An example of a type that isn't Send is Rc<T> because it isn’t thread-safe due to its non-atomic reference counting.

4) What is the relationship between Send and Sync? Does this relate
to Rust's Ownership system somehow?

Send allows ownership transfer between threads, while Sync allows shared references between threads, both of which are related to Rust’s ownership and borrowing system for ensuring safety.

5) Are there any types that could be Send but NOT Sync? Is that even possible?

Yes, types like Cell<T> can be Send but not Sync because they allow ownership transfer but not concurrent access from multiple threads.

6) Could we implement Send ourselves using safe rust? why/why not?

You cannot implement Send manually in safe Rust, as doing so would bypass compile-time safety checks and potentially lead to undefined behavior.