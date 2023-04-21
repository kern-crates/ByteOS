use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use arch::{console_getchar, console_putchar};
use executor::{yield_now, TASK_QUEUE};
use fs::{mount::open, File, FileType, OpenFlags};
use log::debug;

use crate::tasks::add_user_task;

const LF: u8 = b'\n';
const CR: u8 = b'\r';
const DL: u8 = b'\x7f';
const BS: u8 = b'\x08';
const SPACE: u8 = b' ';

fn help() {
    println!("help");
    println!("ls");
    println!("clear");
    println!("exit");
}

fn list_files(file: File, space: usize) {
    for i in file.read_dir().expect("can't read dir") {
        println!("{:<3$}{} {}", "", i.filename, i.len, space);
        if i.file_type == FileType::Directory {
            list_files(
                file.open(&i.filename, OpenFlags::O_RDWR)
                    .expect("can't read dir"),
                space + 4,
            );
        }
    }
}

fn clear() {
    console_putchar(0x1b);
    console_putchar(0x5b);
    console_putchar(0x48);
    console_putchar(0x1b);
    console_putchar(0x5b);
    console_putchar(0x32);
    console_putchar(0x4a);
}

async fn run_libc_test() -> bool {
    let commands = [
        // "./runtest.exe -w entry-static.exe pthread_cancel_points",
        // "./runtest.exe -w entry-static.exe pthread_cancel",
        // "./runtest.exe -w entry-static.exe pthread_cond",
        // "./runtest.exe -w entry-static.exe pthread_tsd",
        // "./runtest.exe -w entry-static.exe setjmp",
        // "./runtest.exe -w entry-static.exe socket",
        // "./runtest.exe -w entry-static.exe strptime",
        // "./runtest.exe -w entry-static.exe utime",
        // "./runtest.exe -w entry-static.exe daemon_failure",
        // "./runtest.exe -w entry-static.exe pthread_robust_detach",
        // "./runtest.exe -w entry-static.exe pthread_cancel_sem_wait",
        // "./runtest.exe -w entry-static.exe pthread_cond_smasher",
        // "./runtest.exe -w entry-static.exe pthread_condattr_setclock",
        // "./runtest.exe -w entry-static.exe pthread_exit_cancel",
        // "./runtest.exe -w entry-static.exe pthread_once_deadlock",
        // "./runtest.exe -w entry-static.exe pthread_rwlock_ebusy",
        // "./runtest.exe -w entry-static.exe rlimit_open_files",
        // "./runtest.exe -w entry-static.exe sigprocmask_internal",
        // "./runtest.exe -w entry-static.exe syscall_sign_extend",
    ];

    for i in commands {
        file_command(i).await
    }

    false
}

async fn run_all() -> bool {
    let commands = [
        "brk",
        "chdir",
        "clone",
        "close",
        "dup",
        "dup2",
        "execve",
        "exit",
        "fork",
        "fstat",
        "getcwd",
        "getpid",
        "getppid",
        "gettimeofday",
        "mkdir_",
        "mmap",
        "mount /dev/sda ./mnt",
        "munmap",
        "open",
        "times",
        "openat",
        "pipe",
        "read",
        "sleep",
        "umount /dev/sda ./mnt",
        "uname",
        "unlink",
        "wait",
        "waitpid",
        "getdents",
        "write",
        "yield",
    ];

    for i in commands {
        file_command(i).await
    }

    return true;
}

async fn file_command(cmd: &str) {
    let mut args: Vec<&str> = cmd.split(" ").filter(|x| *x != "").collect();
    debug!("cmd: {}  args: {:?}", cmd, args);
    let filename = args.drain(..1).last().unwrap();
    let filename = match filename.starts_with("/") {
        true => String::from(filename),
        false => String::from("/") + filename,
    };
    match open(&filename) {
        Ok(_) => {
            info!("exec: {}", filename);
            let mut args_extend = vec![filename.as_str()];
            args_extend.extend(args.into_iter());
            // args.into_iter().for_each(|x| args_extend.push(x));
            add_user_task(&filename, args_extend, Vec::new()).await;

            loop {
                if TASK_QUEUE.lock().len() == 0 {
                    break;
                }
                yield_now().await;
            }
            // syscall(SYS_WAIT4, [0,0,0,0,0,0,0])
            //     .await
            //     .expect("can't wait a pid");
        }
        Err(_) => {
            println!("unknown command: {}", cmd);
        }
    }
}

pub async fn command(cmd: &str) -> bool {
    match cmd.trim() {
        "" => {}
        "help" => help(),
        "ls" => list_files(open("/").expect("can't find mount point at ."), 0),
        "clear" => clear(),
        "exit" => return true,
        "run_all" => return run_all().await,
        _ => file_command(cmd).await,
    }

    false
}

pub async fn initproc() {
    // let names = include_str!("../../../tools/testcase-step2/run-static.sh");
    // for i in names.split('\n') {
    //     command(i).await;
    // }
    run_libc_test().await;
    // command("run_all").await;

    // simple command shell.
    // let mut buffer = Vec::new();
    // let mut new_line = true;
    // loop {
    //     if new_line {
    //         print!("> ");
    //         new_line = false;
    //     }
    //     let c = console_getchar();
    //     if c as i8 != -1 {
    //         match c as u8 {
    //             CR | LF => {
    //                 print!("\n");
    //                 let sign = command(&String::from_utf8_lossy(&buffer).to_string()).await;
    //                 if sign {
    //                     break;
    //                 }
    //                 buffer.clear();
    //                 new_line = true;
    //             }
    //             BS | DL => {
    //                 if buffer.len() > 0 {
    //                     buffer.pop();
    //                     console_putchar(BS);
    //                     console_putchar(SPACE);
    //                     console_putchar(BS);
    //                 }
    //             }
    //             0..30 => {}
    //             _ => {
    //                 buffer.push(c as u8);
    //                 console_putchar(c as u8);
    //             }
    //         }
    //     }
    //     yield_now().await;
    // }
}