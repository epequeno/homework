extern crate nix;
use nix::unistd::{fork, ForkResult};
use nix::fcntl;
use nix::fcntl::OFlag;
use nix::sys::stat;

fn main() {
  /* Write a program that calls fork(). Before calling fork(), 
     have the main process access a variable (e.g., x) and set
     its value to something (e.g., 100). What value is the 
     variable in the child process? What happens to the vari-
     able when both the child and parent change the value of x?
   */
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

  /* Write a program that opens a file (with the open() system 
     call) and then calls fork() to create a new process. Can
     both the child and parent access the file descriptor 
     returned by open()? What happens when they are writing to 
     the file concurrently, i.e., at the same time?
   */
   let access_mode = OFlag::O_CREAT | OFlag::O_RDWR | OFlag::O_NONBLOCK;
   let status_flags = stat::Mode::S_IRUSR | stat::Mode::S_IWUSR;
   let _ = fcntl::open("ch05.txt", access_mode, status_flags).expect("open failed");

}
