extern crate nix;
use nix::unistd::{fork, ForkResult};
use std::thread;
use std::time::Duration;

/* Write another program using fork() . The child process should print 
   “hello”; the parent process should print “goodbye”. You should try to
   ensure that the child process always prints first; can you do this without
   calling wait() in the parent?
 */

fn main() {
  match fork() {
    Ok(ForkResult::Parent {..}) => {
      thread::sleep(Duration::from_millis(100));
      println!("goodbye");
    }
    Ok(ForkResult::Child) => {
      println!("hello");
    }
    Err(e) => {
      println!("fork fail: {}", e);
    }
  }
}