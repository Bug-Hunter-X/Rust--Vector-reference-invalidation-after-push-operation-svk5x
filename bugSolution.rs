fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    
    //Instead of taking a reference directly, clone the value.
    let x = vec[0];
    vec.push(3);
    println!("Value of x: {}", x);
    
    //Or if you need to avoid the clone you can do the following:
    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    {
        let x = &vec2[0];
        println!("Value of x: {}", x);
    }
    vec2.push(3);
} 