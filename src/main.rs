use std::{
    path::PathBuf,
    //env,
};
use nix::{
    fcntl,
    unistd,
    sys::{
        stat,
        mman,
    },
};
use core::num::NonZeroUsize;
use libc::off_t;

#[derive(Debug)]
struct data{
    number0: i32,
    number1: i32,
}

fn main() {
    println!("Hello, world!");
}


fn hola() {
    println!("hola");
}

fn adios() {
    println!("adios");
}

fn open_rm() {
    use stat::Mode;
    use fcntl::OFlag;
    
    /*
    println!("{}", env::var("HOME").unwrap());
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => {
            panic!();
        }
    };
    let mut file_path = PathBuf::from(home_dir);
    */
    let mut file_path = PathBuf::from("./");
    file_path.push("test");
    file_path.push("file");
    let oflag: OFlag = OFlag::O_WRONLY | OFlag::O_CREAT;
    let mode: Mode = Mode::S_IWUSR;
    
    let file = fcntl::open(&file_path, oflag, mode).expect("simple file creation");
    
    unistd::ftruncate(file, 8);
    
    let data = [data{number0:0, number1:1}, data{number0:2, number1:3}];
    /*
    let test = 0x_6C_72_61_63_61_6C_6F_68_u64;
    let slice: &[u8] = unsafe{any_as_u8_slice(&test)};
    */
    let data_slice = unsafe{as_u8_slice(&data)};
    
    let write_result = unistd::write(file, data_slice);
    
}

fn open_wm() {
    use stat::Mode;
    use fcntl::OFlag;
    
    /*
    println!("{}", env::var("HOME").unwrap());
    let home_dir = match env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => {
            panic!();
        }
    };
    let mut file_path = PathBuf::from(home_dir);
    */
    let mut file_path = PathBuf::from("./");
    file_path.push("test");
    file_path.push("file");
    let oflag: OFlag = OFlag::O_RDONLY | OFlag::O_CREAT;
    let mode: Mode = Mode::empty();// = Mode::S_IWUSR;
    
    let file = fcntl::open(&file_path, oflag, mode).expect("simple file creation");
    
    let file_stat = nix::sys::stat::fstat(file).expect("file stat");
    
    println!("{:?}", file_stat.st_size);
    
    
    let addr: Option<NonZeroUsize> = None;
    //let length: i64 = file_stat.st_size;
    let length: NonZeroUsize = NonZeroUsize::new(file_stat.st_size.try_into().unwrap()).unwrap();
    let prot: mman::ProtFlags = mman::ProtFlags::PROT_READ;
    let flags: mman::MapFlags = mman::MapFlags::MAP_SHARED;
    //let : RawFd;
    let offset: off_t = 0;
    
    
    let raw_pointer = unsafe{nix::sys::mman::mmap(addr, length, prot, flags, file, offset.try_into().unwrap()).expect("mmap")};
    println!("{:?}", raw_pointer);
    
    let pointer = raw_pointer as *mut data;
    
    let pointer_ref: &mut data = unsafe{ pointer.as_mut().unwrap()};
    
    println!("{:?}", pointer_ref);
    /*
    let damn: &mut _ = unsafe {&mut *raw_pointer as &mut data};// &mut data;
    
    println!("{:?}", damn);
    */
}


unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    core::slice::from_raw_parts(
        (p as *const T) as *const u8, 
        core::mem::size_of::<T>()
    )
}


#[cfg(test)]
mod file{
    use super::*;
    
    #[test]
    fn open_file() {
        open_rm();
        open_wm();
    }
    
}

/*
#[cfg(test)]
mod adios{
    use super::*;
    
    #[test]
    #[ignore]
    fn dd() {
        adios();
    }
}
*/
