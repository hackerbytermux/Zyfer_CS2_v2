use windows::Win32::Foundation::HANDLE;

use windows::Win32::System::IO::DeviceIoControl;

macro_rules! CTL_CODE {
    ($device_type:expr, $function:expr, $method:expr, $access:expr) => {
        ($device_type << 16) | ($access << 14) | ($function << 2) | $method
    };
}

pub mod driver {
    use std::{mem::MaybeUninit, os::raw::c_void};
    use windows::{
        core::s,
        Win32::{
            Security::SECURITY_ATTRIBUTES,
            Storage::FileSystem::{
                CreateFileA, FILE_ATTRIBUTE_NORMAL, FILE_GENERIC_READ, FILE_SHARE_READ,
                FILE_SHARE_WRITE, OPEN_EXISTING,
            },
        },
    };

    use super::*;

    pub mod codes {

        pub const ATTACH: u32 = CTL_CODE!(0x00000022, 0x696, 0, 0);
        pub const READ: u32 = CTL_CODE!(0x00000022, 0x697, 0, 0);
        pub const WRITE: u32 = CTL_CODE!(0x00000022, 0x698, 0, 0);
        pub const MOD_BASE: u32 = CTL_CODE!(0x00000022, 0x699, 0, 0);
    }

    #[repr(C)]
    pub struct Request {
        pub process_id: HANDLE,
        pub target: *mut std::ffi::c_void,
        pub buffer: *mut std::ffi::c_void,
        pub size: usize,
        pub return_size: usize,
    }

    pub fn attach_to_process(driver_handle: HANDLE, pid: u32) -> Result<(), windows::core::Error> {
        let r = Request {
            process_id: HANDLE(pid as *mut std::ffi::c_void),
            target: std::ptr::null_mut(),
            buffer: std::ptr::null_mut(),
            size: 0,
            return_size: 0,
        };
        let mut returned: u32 = 0; // Change type to u32
        unsafe {
            DeviceIoControl(
                driver_handle,
                codes::ATTACH,
                Some(&r as *const _ as *const c_void), // Wrap in Some and change type to *const c_void
                size_of::<Request>() as u32,
                Some(std::ptr::null_mut()),
                0,
                Some(&mut returned as *mut u32), // Wrap in Some and change type to *mut u32
                Some(std::ptr::null_mut()),
            )
        }
    }

    pub fn read_memory<T>(driver_handle: HANDLE, address: usize) -> T {
        let mut temp: MaybeUninit<T> = MaybeUninit::uninit();
        let r = Request {
            target: address as *mut std::ffi::c_void,
            buffer: temp.as_mut_ptr() as *mut std::ffi::c_void,
            size: size_of::<T>(),
            return_size: 0,
            process_id: HANDLE(0 as *mut std::ffi::c_void),
        };
        let mut returned: u32 = 0;
        unsafe {
            let _ = DeviceIoControl(
                driver_handle,
                codes::READ,
                Some(&r as *const _ as *mut _),
                size_of::<Request>() as u32,
                Some(&r as *const _ as *mut _),
                size_of::<Request>() as u32,
                Some(&mut returned),
                Some(std::ptr::null_mut()),
            );
            temp.assume_init()
        }
    }

    pub fn read_memory_raw(driver_handle: HANDLE, address: usize, size: usize) -> Vec<u8> {
        let mut temp = vec![0u8; size];

        let r = Request {
            process_id: HANDLE::default(),
            target: address as *mut std::ffi::c_void,
            buffer: temp.as_mut_ptr() as *mut std::ffi::c_void,
            size: size,
            return_size: 0,
        };

        let mut returned: u32 = 0;
        unsafe {
            let _ = DeviceIoControl(
                driver_handle,
                codes::READ,
                Some(&r as *const _ as *mut _),
                std::mem::size_of::<Request>() as u32,
                Some(&r as *const _ as *mut _),
                std::mem::size_of::<Request>() as u32,
                Some(&mut returned),
                Some(std::ptr::null_mut()),
            );
        }

        temp
    }
    /*
    pub fn write_memory<T>(driver_handle: HANDLE, address: usize, value: &T) {
        let mut r = Request {
            process_id: HANDLE::default(),
            target: PVOID(address as isize),
            buffer: value as *const _ as PVOID,
            size: std::mem::size_of::<T>() as SIZE_T,
            return_size: 0,
        };

        unsafe {
            DeviceIoControl(
                driver_handle,
                codes::WRITE,
                &mut r as *mut _ as *mut _,
                std::mem::size_of::<Request>() as u32,
                &mut r as *mut _ as *mut _,
                std::mem::size_of::<Request>() as u32,
                None,
                None,
            );
        }
    }
    */
    pub fn get_module_base(driver_handle: HANDLE) -> *mut std::ffi::c_void {
        let temp: *mut std::ffi::c_void = std::ptr::null_mut();

        let r = Request {
            buffer: temp,
            target: std::ptr::null_mut(),
            size: size_of::<*mut std::ffi::c_void>(),
            return_size: 0,
            process_id: windows::Win32::Foundation::HANDLE(std::ptr::null_mut()),
        };
        let mut returned: u32 = 0;
        unsafe {
            let _ = DeviceIoControl(
                driver_handle,
                codes::MOD_BASE,
                Some(&r as *const _ as *mut _),
                size_of::<Request>() as u32,
                Some(&r as *const _ as *mut _),
                size_of::<Request>() as u32,
                Some(&mut returned),
                Some(std::ptr::null_mut()),
            );
            r.buffer
        }
    }

    pub fn connect_to_driver() -> Result<windows::Win32::Foundation::HANDLE, windows::core::Error> {
        unsafe {
            let security_attributes: *const SECURITY_ATTRIBUTES = std::ptr::null();

            let template_file: HANDLE = windows::Win32::Foundation::HANDLE(std::ptr::null_mut());

            // Call CreateFileW
            let driver_handle = CreateFileA(
                s!(r"\\.\Zyfer"),
                FILE_GENERIC_READ.0,                // Desired access
                FILE_SHARE_READ | FILE_SHARE_WRITE, // Share mode
                Some(security_attributes),
                OPEN_EXISTING,         // Creation disposition
                FILE_ATTRIBUTE_NORMAL, // Flags and attributes
                template_file,         // Template file
            );
            driver_handle
        }
    }
}
