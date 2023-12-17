// Assigning value of one variable to another variable.

fn main() 
{
    let v = vec![101, 250, 330, 400];
    let v2 = [101, 250, 330, 400];
    // vector v owns the object in heap

    // only a single variable own the heap memeory at any given time
    let _v3 = v; 
    let _v4 = v2;
    // If you use "&" before v, v3 would be borrowing the values of v.
    // E.g let v3 = &v;

    // here two variables owns heap value'
    // two pointers to the same content is not allowed in rust

    // Rust is very smart in terms of memory access ,so it detects a race condition
    // as two variables point to same heap

    println!("{:?}", v);
    // Note: Error has occurred, why?
    // Because variable v3 was newly assigned as variable v,
    // Making v with no value and vectors are non-primitive.

    println!("{:?}", v2);
    // Note: No error occurred, why?
    // Because an array is primitive.
}
