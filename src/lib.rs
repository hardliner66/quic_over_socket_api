extern crate libc;

use libc::{c_int, c_void, sockaddr, socklen_t, ssize_t /*, AF_INET, SOCK_DGRAM*/};
use std::ffi::CString;

lazy_static::lazy_static! {
    static ref REAL_SOCKET: fn(c_int, c_int, c_int) -> c_int = unsafe {
        let func_ptr = libc::dlsym(libc::RTLD_NEXT, CString::new("socket").unwrap().into_raw());
        std::mem::transmute(func_ptr)
    };
    static ref REAL_BIND: fn(c_int, *const sockaddr, socklen_t) -> c_int = unsafe {
        let func_ptr = libc::dlsym(libc::RTLD_NEXT, CString::new("bind").unwrap().into_raw());
        std::mem::transmute(func_ptr)
    };
    static ref REAL_RECVFROM: fn(c_int, *mut c_void, usize, c_int, *mut sockaddr, *mut socklen_t) -> ssize_t = unsafe {
        let func_ptr = libc::dlsym(libc::RTLD_NEXT, CString::new("recvfrom").unwrap().into_raw());
        std::mem::transmute(func_ptr)
    };
    static ref REAL_SENDTO: fn(c_int, *const c_void, usize, c_int, *const sockaddr, socklen_t) -> ssize_t = unsafe {
        let func_ptr = libc::dlsym(libc::RTLD_NEXT, CString::new("sendto").unwrap().into_raw());
        std::mem::transmute(func_ptr)
    };
}

#[no_mangle]
pub unsafe extern "C" fn socket(domain: c_int, type_: c_int, protocol: c_int) -> c_int {
    let result = REAL_SOCKET(domain, type_, protocol);
    println!("socket({domain}, {type_}, {protocol}) => {result}");
    result
}

#[no_mangle]
pub extern "C" fn bind(sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> c_int {
    let result = REAL_BIND(sockfd, addr, addrlen);
    println!("bind({sockfd}, {addr:?}, {addrlen}) => {result}");
    result
}

#[no_mangle]
pub extern "C" fn sendto(
    sockfd: c_int,
    buf: *const c_void,
    len: usize,
    flags: c_int,
    dest_addr: *const sockaddr,
    addrlen: socklen_t,
) -> ssize_t {
    let result = REAL_SENDTO(sockfd, buf, len, flags, dest_addr, addrlen);
    println!("sendto({sockfd}, {buf:?}, {len}, {flags}, {dest_addr:?}, {addrlen}) => {result}");
    result
}

#[no_mangle]
pub extern "C" fn recvfrom(
    sockfd: c_int,
    buf: *mut c_void,
    len: usize,
    flags: c_int,
    src_addr: *mut sockaddr,
    addrlen: *mut socklen_t,
) -> ssize_t {
    let result = REAL_RECVFROM(sockfd, buf, len, flags, src_addr, addrlen);
    println!("recvfrom({sockfd}, {buf:?}, {len}, {flags}, {src_addr:?}, {addrlen:?}) => {result}");
    result
}
