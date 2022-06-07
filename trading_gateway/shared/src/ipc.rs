use crate::errors::MGError;
use fehler::throws;
use std::fs::create_dir;
use std::path::Path;

pub static WORKDIR: &str = "/tmp/trading_gateway";
pub static MG_SOCK: &str = "ipc:///tmp/trading_gateway/mg.sock";
pub static MD_SOCK: &str = "ipc:///tmp/trading_gateway/md.sock";

#[throws(MGError)]
pub fn ensure_workdir() {
    if !Path::new(WORKDIR).exists() {
        create_dir("/tmp/trading_gateway")?;
    }
}
