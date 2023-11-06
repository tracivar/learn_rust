use regex::Regex;

fn main() {
    let edge = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36 Edg/118.0.2088.76";
    let safari = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Version/17.1 Safari/537.36 Edg/118.0.2088.76";
    let useragent = safari;

    let has_others = Regex::new(r"(?i)(chrome|android)").unwrap().is_match(useragent);
    let has_safari = Regex::new(r"(?i)safari").unwrap().is_match(useragent);
    if !has_others && has_safari {
        let version_regex = Regex::new(r"Version\/(\S)+").unwrap();
        let version = version_regex.find(useragent).unwrap().as_str();
        println!("Safari {version}");
    } else {
        println!("This is not a Safari browser");
    }
}
