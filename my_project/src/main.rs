fn main() {
    let mut stack = vec![]; //vector 

    stack.push(10);
    stack.push(11);
    stack.push(12);
    stack.push(13);

    while !stack.is_empty(){
        let x = stack.pop().unwrap();
        print!("{} ", x); 
    }
    println!(""); 


}
