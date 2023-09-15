use {std::process::exit, ureq};

const VERSION: &str = include_str!("../VERSION");

fn is_latest(url: &str) -> bool { match ureq::get(url).call() {
    Err(e)=>{ eprintln!("Failed: {}", e); exit(-1); }
    Ok(r) => { match r.into_string() {
        Err(e)=>{ eprintln!("Failed: {}", e); exit(-1); },
        Ok(version) => { match version.as_str() {
            VERSION => true, _ => { eprintln!("Local version: {}\nFetched version: {}\nDifferences found.", VERSION, version); false
}}}}}}}

pub fn update(url: &str) { match is_latest(url) {
    true => { println!("Currently running on the latest update.") },
    false => { println!("Update function has not been implemented yet."); }
}}