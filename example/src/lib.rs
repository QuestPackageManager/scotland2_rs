use scotland2_rs::{ModInfoBuf, scotland2_raw::CModInfo};

static MOD_INFO: ModInfoBuf = ModInfoBuf {
    id: "example".to_string(),
    version: "0.1.0".to_string(),
    version_long: 1,
};

extern "C" fn setup(mod_info: &mut CModInfo) {
    *mod_info = MOD_INFO.clone().into();

    println!("Setup: {}", mod_info);
}

extern "C" fn early_load() {
    println!("Early load {MOD_INFO}");
}
extern "C" fn late_load() {
    println!("Late load {MOD_INFO}");
}
