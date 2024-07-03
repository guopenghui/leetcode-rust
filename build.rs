// use std::env;
// use std::path::Path;

fn main() {
    impl_main();
}

fn impl_main() {
    // 指定模块文件夹的路径
    let module_dir = "src/problems";

    // 获取模块文件夹下的所有 .rs 文件
    let mut paths = std::fs::read_dir(module_dir)
        .expect("Failed to read module directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension() == Some("rs".as_ref()) {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();

    // 根据题号排个序
    fn get_id(path: &std::path::PathBuf) -> Option<u32> {
        let (id, _) = path.file_stem()?.to_str()?.split_once('.')?;
        id.parse::<u32>().ok()
    }
    paths.sort_by_key(get_id);
    // 生成 mod.rs 文件
    // let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR environment variable");
    let mod_rs_path = "src/problems.rs".to_string();
    // let mod_rs_path = Path::new(&out_dir).join("mod.rs");
    let mut mod_rs_content = String::new();

    for path in paths {
        if let Some(file_name) = path.file_stem() {
            if let Some(file_name) = file_name.to_str() {
                let (id, _) = file_name.split_once('.').unwrap();
                let id = id.parse::<i32>().unwrap();
                mod_rs_content.push_str(&format!(
                    "#[path=\"problems/{file_name}.rs\"]\npub mod q_{id:04};\n"
                ));
            }
        }
    }

    std::fs::write(&mod_rs_path, mod_rs_content).expect("Failed to write mod.rs");
}
