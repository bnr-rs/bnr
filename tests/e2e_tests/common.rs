use std::sync::{Mutex, MutexGuard};
use bnr::Result;

static INIT: Mutex<()> = Mutex::new(());

pub fn init() -> Result<MutexGuard<'static, ()>> {
    Ok(INIT.lock()?)
}
