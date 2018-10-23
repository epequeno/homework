extern crate nix;
use nix::unistd::{fork, ForkResult};
use nix::sys::wait::wait;

/* Now write a program that uses wait() to wait for the child process to finish
   in the parent. What does wait() return? What happens if you use wait() in 
   the child?
 */

fn main() {
  match fork() {
    Ok(ForkResult::Parent {..}) => {
      match wait() {
        Ok(res) => println!("parent {:?}", res),
        Err(e) => println!("parent err: {}", e)
      }
    }
    Ok(ForkResult::Child) => {
      match wait() {
        Ok(res) => println!("child {:?}", res),
        Err(e) => println!("child err: {}", e)
      }
    }
    Err(e) => {
      println!("fork fail: {}", e);
    }
  }  
}