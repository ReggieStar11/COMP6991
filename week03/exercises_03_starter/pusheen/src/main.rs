fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Since a is &mut vec, only one can exsit at any given time
    // since the liftetime of a does not exist before b is called
    // the code will not be able to comple as vec is being borrowed more than once at a time
  
    let a = &mut vec.clone();
    let b = &mut vec;

    // So now with clone a is mutuable reference to the cloned vector,
    // whilst b is a referenced to the originla vector, meaning the original vector is not existing  twice
    
    a.push(11);
    b.push(12);
}
