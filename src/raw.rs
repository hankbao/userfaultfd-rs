use libc::{c_int, c_long, syscall, SYS_userfaultfd, INT_MAX};
pub use userfaultfd_sys::*;

pub unsafe fn userfaultfd(flags: c_int) -> c_int {
    let fd = syscall(SYS_userfaultfd, flags as c_long);
    if fd > INT_MAX as c_long {
        panic!("fd doesn't fit in a c_int");
    } else {
        fd as c_int
    }
}

ioctl_readwrite!(api, UFFDIO, _UFFDIO_API, uffdio_api);
ioctl_readwrite!(register, UFFDIO, _UFFDIO_REGISTER, uffdio_register);
ioctl_read!(unregister, UFFDIO, _UFFDIO_UNREGISTER, uffdio_range);
ioctl_read!(wake, UFFDIO, _UFFDIO_WAKE, uffdio_range);
ioctl_readwrite!(copy, UFFDIO, _UFFDIO_COPY, uffdio_copy);
ioctl_readwrite!(zeropage, UFFDIO, _UFFDIO_ZEROPAGE, uffdio_zeropage);
