
fn main() {
    if zkcred_core::credit_issued() {
        println!("zkCred-Q CLI: IOU issued securely!");
    }
}
