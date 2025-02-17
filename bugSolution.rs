fn main() {
    let mut x = 5;
    { // Scope to limit the mutable borrow
        let y = &mut x; 
        *y += 1; 
    }
    { // New scope for the other mutable borrow
        let z = &mut x; 
        *z += 1;
    }
    println!("x = {}", x);
}
