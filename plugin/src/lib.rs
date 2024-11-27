use extism_pdk::{debug,plugin_fn,FnResult};

#[plugin_fn]
pub extern "C" fn test() -> FnResult<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        debug!("here1");
        tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
        debug!("here2");
    });
    Ok(())
}
