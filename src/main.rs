use regex::Regex;

fn main() {
    let edge = String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.76");
    let safari = String::from("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Version/17.1 Safari/537.36 Edg/118.0.2088.76");
    is_safari_low_version(&safari);
}

fn is_safari_low_version(useragent: &String) -> bool {
    let has_others = Regex::new(r"(?i)(chrome|android)").unwrap().is_match(useragent);
    let has_safari = Regex::new(r"(?i)safari").unwrap().is_match(useragent);
    if !has_others && has_safari {
        let version_regex = Regex::new(r"Version\/(?<version>\S+)").unwrap();
        let Some(caps) = version_regex.captures(useragent) else {
            println!("No safari version match!");
            return false;
        };
        let version_str = &caps["version"];
        let version_as_num: f32 = version_str.parse().unwrap();
        if version_as_num < 17.1 {
            println!("Safari {version_as_num}");
            return true;
        } else {
            println!("Not Safari below 17.1");
        }
    } 
    println!("Not a Safari browser");
    return false;
}
