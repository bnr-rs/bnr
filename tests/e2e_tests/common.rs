use bnr::Result;
use std::sync::{Mutex, MutexGuard};

static INIT: Mutex<()> = Mutex::new(());

pub fn init() -> Result<MutexGuard<'static, ()>> {
    Ok(INIT.lock()?)
}
