extern crate nix;
use nix::unistd::{execvp,
                  execv,
                  execve,
                  fork, 
                  ForkResult};
use std::ffi::CString;

/* Write a program that calls fork() and then calls some form of exec() to run
   the program /bin/ls. See if you can try all of the variants of exec(), 
   including (on Linux) execl(), execle(), execlp(), execv(), execvp(), and 
   execvpe(). Why do you think there are so many variants of the same basic 
   call?
 */

 // note: On success, this function (exec*) will not return.

 fn main() {
  match fork() {
    Ok(ForkResult::Parent {..}) => {
      let prog = CString::new("/bin/ls").expect("new string failed");
      let args = &[CString::new("").expect("new string failed")];
      let env = &[CString::new("").expect("new string failed")];
      let _ = execvp(&prog, args);
      let _ = execv(&prog, args);
      let _ = execve(&prog, args, env);
    }
    Ok(ForkResult::Child) => {
      println!("hello");
    }
    Err(e) => {
      println!("fork fail: {}", e);
    }
  }
 }