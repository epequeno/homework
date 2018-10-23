extern crate nix;
use nix::unistd::{getpid, fork, ForkResult};
use nix::sys::wait::{waitpid, WaitPidFlag};

/* Write a slight modification of the previous program, this time using 
   waitpid() instead of wait(). When would waitpid() be useful?
 */

fn main() {
  match fork() {
    Ok(ForkResult::Parent { child }) => {
      let pid = getpid();
      match waitpid(child, Some(WaitPidFlag::empty())) {
        Ok(res) => println!("parent({}) {:?}", &pid, res),
        Err(e) => println!("parent err: {}", e)
      }
    }
    Ok(ForkResult::Child) => {
      let pid = getpid();
      println!("child({})", pid)
    }
    Err(e) => {
      println!("fork fail: {}", e);
    }
  }    
}