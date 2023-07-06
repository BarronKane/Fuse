#[cfg(target_os = "windows")]
use windows;

use std::error::Error;

#[derive(Debug)]
pub struct InstanceError(String);
impl Error for InstanceError {}
impl std::fmt::Display for InstanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "INSTANCING: {}", self.0)
    }
}
pub type Result<T> = std::result::Result<T, InstanceError>;

pub use self::instance::*;

#[cfg(target_os = "windows")]
mod instance {
    use super::InstanceError;
    use super::Result;

    use windows::Win32::Foundation::BOOL;
    use windows::Win32::Security::SECURITY_ATTRIBUTES;
    use windows::Win32::{
        Foundation,
        Foundation::HANDLE,

        System::Threading,
    };

    use std::collections::HashMap;

    pub struct InstanceMap {
        instances: HashMap<String, Instance>
    }

    impl InstanceMap {
        pub fn new() -> InstanceMap {
            InstanceMap {
                instances: HashMap::new(),
            }
        }

        pub fn try_new(name: &str) -> Result<InstanceMap> {

            let mut instance_map = InstanceMap {
                instances: HashMap::new(),
            };

            let instance = Instance::new(name)?;
            
            instance_map.push(name, instance)?;

            Ok(instance_map)
        }

        pub fn try_push(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) {
                Err(InstanceError("Instance exists!".to_string()))
            } else {
                let instance = Instance::new(name)?;
                Ok(self.push(name, instance)?)
            }
        }

        fn push(&mut self, name: &str, instance: Instance) -> Result<&mut Self> {
            let i = self.instances.insert(name.to_string(), instance);
            match i {
                None => Ok(self),
                Some(_) => {
                    panic!("Tried to instance something already instanced! This should be unreachable.")
                }
            }
        }

        pub fn is_mapped(&self, name: &str) -> bool {
            if !self.instances.contains_key(name) {
                false
            } else {
                true
            }
        }

        pub fn is_active(&self, name: &str) -> Result<bool> {
            if !self.instances.contains_key(name) {
                Err(InstanceError("Instance is not mapped.".to_string()))
            } else {
                Ok(self.instances[name].exists())
            }
        }

        pub fn release_instance(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) && self.is_active(name).unwrap() {
                self.instances.get_mut(name).unwrap().release();
                Ok(self)
            } else {
                Err(InstanceError("Instance doesn't exist!".to_string()))
            }            
        }

        pub fn restart_instance(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) && !self.is_active(name).unwrap() {
                self.instances.get_mut(name).unwrap().create_handle(name)?;
                Ok(self)
            } else {
                Err(InstanceError("Could not reinstate instance mutex!".to_string()))
            }
        }
    }

    struct Instance {
        handle: Option<HANDLE>,
    }

    unsafe impl Send for Instance {}
    unsafe impl Sync for Instance {}

    impl Instance {
        fn new(name: &str) -> Result<Self> {
            let mut instance = Instance { handle: None };

            instance.create_handle(name)?;
            Ok(instance)
        }

        fn exists(&self) -> bool {
            self.handle.is_some()
        }

        fn release(&mut self) {
            if let Some(h) = self.handle.take() {
                unsafe {
                    Foundation::CloseHandle(h);
                }
            }
        }

        fn create_handle(&mut self, name: &str) -> Result<&Self> {
            

            unsafe {
                const sa: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES
                {
                    bInheritHandle: BOOL(0),
                    lpSecurityDescriptor: std::ptr::null_mut(),
                    nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32
                };
                let test_name = name.clone();
                let utf16name = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_mut_ptr();
                let pcwstrname = windows::core::PCWSTR::from_raw(utf16name);

                let handle = Threading::CreateMutexW(Some(&sa), Foundation::BOOL(0), pcwstrname);
                let lerr = Foundation::GetLastError();

                if handle.is_err() {
                    Err(InstanceError("Windows handle invalid!".to_string()))
                } else if lerr.0 != 0 {
                    /*
                    Foundation::CloseHandle(handle);
                    Ok(Instance{ handle: None })
                    */
                    Err(InstanceError("Handle exists! Is process already running?".to_string()))
                } else {
                    self.handle = Some(handle.unwrap());
                    Ok(self)
                }
            }
        }
    }

    impl Drop for Instance {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                unsafe {
                    Foundation::CloseHandle(handle);
                }
            }
        }
    }
}

#[cfg(target_os = "linux")]
mod instance {
    use super::InstanceError;
    use super::Result;

    use nix::sys::socket::{self, UnixAddr};
    use nix::unistd;
    use std::os::unix::prelude::RawFd;

    use std::collections::HashMap;

    pub struct InstanceMap {
        instances: HashMap<String, Instance>
    }

    impl InstanceMap {
        pub fn new() -> InstanceMap {
            InstanceMap {
                instances: HashMap::new(),
            }
        }

        pub fn try_new(name: &str) -> Result<InstanceMap> {

            let mut instance_map = InstanceMap {
                instances: HashMap::new(),
            };

            let instance = Instance::new(name)?;
            
            instance_map.push(name, instance)?;

            Ok(instance_map)
        }

        pub fn try_push(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) {
                Err(InstanceError("Instance exists!".to_string()))
            } else {
                let instance = Instance::new(name)?;
                Ok(self.push(name, instance)?)
            }
        }

        fn push(&mut self, name: &str, instance: Instance) -> Result<&mut Self> {
            let i = self.instances.insert(name.to_string(), instance);
            match i {
                None => Ok(self),
                Some(_) => {
                    panic!("Tried to instance something already instanced! This should be unreachable.")
                }
            }
        }

        pub fn is_mapped(&self, name: &str) -> bool {
            if !self.instances.contains_key(name) {
                false
            } else {
                true
            }
        }

        pub fn is_active(&self, name: &str) -> Result<bool> {
            if !self.instances.contains_key(name) {
                Err(InstanceError("Instance is not mapped.".to_string()))
            } else {
                Ok(self.instances[name].exists())
            }
        }

        pub fn release_instance(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) && self.is_active(name).unwrap() {
                self.instances.get_mut(name).unwrap().release();
                Ok(self)
            } else {
                Err(InstanceError("Instance doesn't exist!".to_string()))
            }            
        }

        pub fn restart_instance(&mut self, name: &str) -> Result<&mut Self> {
            if self.is_mapped(name) && !self.is_active(name).unwrap() {
                self.instances.get_mut(name).unwrap().create_handle(name)?;
                Ok(self)
            } else {
                Err(InstanceError("Could not reinstate instance mutex!".to_string()))
            }
        }
    }

    struct Instance {
        handle: Option<RawFd>,
    }

    unsafe impl Send for Instance {}
    unsafe impl Sync for Instance {}

    impl Instance {
        fn new(name: &str) -> Result<Self> {
            let mut instance = Instance { handle: None };

            instance.create_handle(name)?;
            Ok(instance)
        }

        fn exists(&self) -> bool {
            self.handle.is_some()
        }

        fn release(&mut self) {
            if let Some(h) = self.handle.take() {
                
            }
        }

        fn create_handle(&mut self, name: &str) -> Result<&Self> {
            let addr = UnixAddr::new_abstract(name.as_bytes()).expect("Unable to create unix address.");
            let sock = socket::socket(
                socket::AddressFamily::Unix,
                socket::SockType::Stream,
                socket::SockFlag::empty(),
                None,
            )?;

            /*
            unsafe {
                const sa: SECURITY_ATTRIBUTES = SECURITY_ATTRIBUTES
                {
                    bInheritHandle: BOOL(0),
                    lpSecurityDescriptor: std::ptr::null_mut(),
                    nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32
                };
                let test_name = name.clone();
                let utf16name = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>().as_mut_ptr();
                let pcwstrname = windows::core::PCWSTR::from_raw(utf16name);

                let handle = Threading::CreateMutexW(Some(&sa), Foundation::BOOL(0), pcwstrname);
                let lerr = Foundation::GetLastError();

                if handle.is_err() {
                    Err(InstanceError("Windows handle invalid!".to_string()))
                } else if lerr.0 != 0 {
                    /*
                    Foundation::CloseHandle(handle);
                    Ok(Instance{ handle: None })
                    */
                    Err(InstanceError("Handle exists! Is process already running?".to_string()))
                } else {
                    self.handle = Some(handle.unwrap());
                    Ok(self)
                }
            }
            */

        }
    }

    impl Drop for Instance {
        fn drop(&mut self) {
            if let Some(handle) = self.handle.take() {
                unsafe {
                    Foundation::CloseHandle(handle);
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
mod instance {
    use super::InstanceError;
    use super::Result;

    use libc::{__error, flock, EWOULDBLOCK, LOCK_EX, LOCK_NB};
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    use std::path::Path;

    use std::collections::HashMap;


}
