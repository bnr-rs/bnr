use bnr::Result;
use std::sync::{Mutex, MutexGuard};

static INIT: Mutex<()> = Mutex::new(());

pub fn init() -> Result<MutexGuard<'static, ()>> {
    env_logger::try_init().ok();
    Ok(INIT.lock()?)
}
