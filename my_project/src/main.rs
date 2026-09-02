fn main() {
    let x = rand::random_range(1..=1000);
    
    println!("{}", x);

    for i in 1..=1000 {
        println!("{} {}", i , check(i, x));

        if check(i, x) == 0 {
        println!("we found it! {}", i); 
        }
    }
    

}

fn check (guess: i32, secret: i32) -> i32 {
    if guess == secret { 0 }
    else if guess > secret { 1 }
    else { -1 }
}