#![no_main]

use optee_utee::{
    ta_close_session, ta_create, ta_destroy, ta_invoke_command, ta_open_session, trace_println,
};
use optee_utee::{Error, ErrorKind, Parameters, Result};

#[ta_create]
fn create() -> Result<()> {
    trace_println!("[+] TA create");
    Ok(())
}

#[ta_open_session]
fn open_session(_params: &mut Parameters) -> Result<()> {
    trace_println!("[+] TA open session");
    Ok(())
}

#[ta_close_session]
fn close_session() {
    trace_println!("[+] TA close session");
}

#[ta_destroy]
fn destroy() {
    trace_println!("[+] TA destroy");
}

#[ta_invoke_command]
fn invoke_command(
    cmd_id: u32,
    params: &mut Parameters,
) -> Result<()> {
    trace_println!("[+] TA invoke command");
    match cmd_id {
        TA_HELLO_WORLD_CMD_INC_VALUE => {
            let ori_value = params.param_0.get_value_a()?;
            params.param_0.set_value_a(ori_value + 100)?;
            Ok(())
        }
        TA_HELLO_WORLD_CMD_DEC_VALUE => {
            let ori_value = params.param_0.get_value_a()?;
            params.param_0.set_value_a(ori_value - 100)?;
            Ok(())
        }
        _ => Err(Error::new(ErrorKind::BadParameters)),
    }
}

// TA configurations
const TA_FLAGS: libc::uint32_t = 0;
const TA_DATA_SIZE: libc::uint32_t = 32 * 1024;
const TA_STACK_SIZE: libc::uint32_t = 2 * 1024;
const TA_VERSION: &[u8] = b"0.1\0";
const TA_DESCRIPTION: &[u8] = b"This is an hello world example.\0";
const EXT_PROP_VALUE_1: &[u8] = b"Hello World TA\0";
const EXT_PROP_VALUE_2: libc::uint32_t = 0x0010;
const TRACE_LEVEL: libc::c_int = 4;
const TRACE_EXT_PREFIX: &[u8] = b"TA\0";
const TA_FRAMEWORK_STACK_SIZE: libc::uint32_t = 2048;

include!(concat!(env!("OUT_DIR"), "/user_ta_header.rs"));