use extism_pdk::{debug,plugin_fn,FnResult};

#[plugin_fn]
pub extern "C" fn test() -> FnResult<()> {
    debug!("{:?}", std::time::SystemTime::now());
    std::thread::sleep(std::time::Duration::from_millis(2000));
    debug!("{:?}", std::time::SystemTime::now());
    Ok(())
}
