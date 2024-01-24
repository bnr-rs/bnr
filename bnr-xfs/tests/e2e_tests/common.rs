use std::sync::{Mutex, MutexGuard};

use bnr_xfs::Result;

static INIT_LOCK: Mutex<()> = Mutex::new(());

pub fn init() -> Result<MutexGuard<'static, ()>> {
    env_logger::try_init().ok();
    Ok(INIT_LOCK.lock()?)
}
