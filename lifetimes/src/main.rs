
// The lifetimes prevent dangling references.
fn main() {
    let r;
    {
        // Here we are in another scope.
        let x = 5;
        r = &x;
    } 
    println!("r: {}", r);
    // The program is rejected because r has a longer lifetime than
    // the variable x. We can't borrow a variable that
    // has a lifetime shorter than the assigned variable.
}
