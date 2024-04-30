use std::ffi::c_char;

use machineid_rs::{Encryption, HWIDComponent, IdBuilder};

mod utils;

#[repr(C)]
pub enum CComponentCode {
    SystemID,
    OSName,
    CPUCores,
    CPUID,
    DriveSerial,
    MacAddress,
    FileToken(*const c_char),
    Username,
    MachineName,
}

static mut COMPONENTS: Vec<CComponentCode> = Vec::new();
static mut FILE_TOKEN: String = String::new();

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)] // what can I do?
pub extern "C" fn setup_builder(component_ptr: *const CComponentCode, len: usize) {
    let components = unsafe { std::slice::from_raw_parts(component_ptr, len) };

    for component in components {
        unsafe {
            match component {
                CComponentCode::SystemID => COMPONENTS.push(CComponentCode::SystemID),
                CComponentCode::OSName => COMPONENTS.push(CComponentCode::OSName),
                CComponentCode::CPUCores => COMPONENTS.push(CComponentCode::CPUCores),
                CComponentCode::CPUID => COMPONENTS.push(CComponentCode::CPUID),
                CComponentCode::DriveSerial => COMPONENTS.push(CComponentCode::DriveSerial),
                CComponentCode::MacAddress => COMPONENTS.push(CComponentCode::MacAddress),
                CComponentCode::FileToken(file) => {
                    COMPONENTS.push(CComponentCode::FileToken(*file));
                    FILE_TOKEN = utils::cstr_to_string(*file);
                }
                CComponentCode::Username => COMPONENTS.push(CComponentCode::Username),
                CComponentCode::MachineName => COMPONENTS.push(CComponentCode::MachineName),
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn get_machine_id(secret_key: *const u8) -> *mut c_char {
    let mut builder = IdBuilder::new(Encryption::SHA256);

    for component in unsafe { COMPONENTS.iter() } {
        match component {
            CComponentCode::SystemID => builder.add_component(HWIDComponent::SystemID),
            CComponentCode::OSName => builder.add_component(HWIDComponent::OSName),
            CComponentCode::CPUCores => builder.add_component(HWIDComponent::CPUCores),
            CComponentCode::CPUID => builder.add_component(HWIDComponent::CPUID),
            CComponentCode::DriveSerial => builder.add_component(HWIDComponent::DriveSerial),
            CComponentCode::MacAddress => builder.add_component(HWIDComponent::MacAddress),
            CComponentCode::FileToken(_) => unsafe {
                builder.add_component(HWIDComponent::FileToken(FILE_TOKEN.as_str()))
            },
            CComponentCode::Username => builder.add_component(HWIDComponent::Username),
            CComponentCode::MachineName => builder.add_component(HWIDComponent::MachineName),
        };
    }

    // Build secret key
    let secret_key = utils::cstr_to_string(secret_key as _);
    let harware_id = builder.build(&secret_key).unwrap_or("ERROR".to_owned());
    utils::c_string(harware_id.as_ref())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
} */
