extern crate nix;
use nix::unistd::{fork, ForkResult};

/* Write a program that calls fork(). Before calling fork(), 
   have the main process access a variable (e.g., x) and set
   its value to something (e.g., 100). What value is the 
   variable in the child process? What happens to the vari-
   able when both the child and parent change the value of x?
 */

fn main() {
  let mut x = 100;  
  match fork() {
    Ok(ForkResult::Parent { .. }) => {
      x += 10;
      println!("parent: {}", x);
    }
    Ok(ForkResult::Child) => {
      x += 10;
      println!("child: {}", x);
    }
    Err(_) => println!("fail")
  }
}