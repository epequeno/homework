extern crate nix;
use nix::unistd::{fork, ForkResult};
use nix::fcntl;
use nix::fcntl::OFlag;
use nix::sys::stat;

/* Write a program that opens a file (with the open() system 
   call) and then calls fork() to create a new process. Can
   both the child and parent access the file descriptor 
   returned by open()? What happens when they are writing to 
   the file concurrently, i.e., at the same time?
 */

fn main() {
   let access_mode = OFlag::O_CREAT | OFlag::O_RDWR | OFlag::O_NONBLOCK;
   let status_flags = stat::Mode::S_IRUSR | stat::Mode::S_IWUSR;
   let f = fcntl::open("ch05.txt", access_mode, status_flags).expect("open failed");

  match fork() {
    Ok(ForkResult::Parent { .. }) => {
      for _ in 0..100 {
        let data = b"parent\n";
        let _ = nix::unistd::write(f, data);
      }
    }
    Ok(ForkResult::Child) => {
      for _ in 0..100 {
        let data = b"child\n";
        let _ = nix::unistd::write(f, data);
      }
    }
    Err(_) => println!("fork fail")
  }   
}