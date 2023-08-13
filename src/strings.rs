use std::process::exit;
const VERSION: &str = include_str!("../VERSION");

// Best reason to not collaborate with me: this.

pub fn fetch(text: &str) -> String { let identifier=text.split('.').collect::<Vec<&str>>();
    match identifier.len() { 2|3=>(), _ => { eprintln!("Passed value has invalid identifier count."); exit(-1); }}
    let n=String::new(); let u=""; let mut c=false;
    /*   Empty string + c are for duct taping stuff like Russia duct taping drones
         `u` and `c` are just `n` if you look at them in a certain rotation
    -->  Identifier[0]  Categories with/without symbols
         Identifier[1]  Further categories
                        * Non-existent sometimes
    */// Identifier[2]  Actual string name
    let final_=match identifier[0] {
/*  */  "commands" => match identifier[1] {
            "help"=>format!("\x1b[1mOkeanos {}\x1b[0m\nA shell project done \x1b[4mterrible\x1b[0m.\n\nProgram parameters:\n    \x1b[1mOkeanos\x1b[0m [FLAGS]\n\nProgram flags:\n    \x1b[1m--help\x1b[0m & \x1b[1m-h\x1b[0m  ::  Shows Okeanos's description and usage.\n    \x1b[1m--version\x1b[0m & \x1b[1m-v\x1b[0m  ::  Outputs current version of Okeanos.", VERSION),
            "version"=>VERSION.to_string(),
        _=>n },
/*  */  "information" => format!("\x1b[1;36m!!\x1b[0m {}", match identifier[1] {
            "profile" => match identifier[2] {
                "userMissing"=>"No local Okeanos profile could be found. Generate one?",
                "systemMissing"=>"No system Okeanos profile could be found. Is this your first time running?",
            _=>{c=true;u}},
            "generation" => match identifier[2] {
                "request"=>"Would you want to generate a new, default, system-wide Okeanos profile?",
                "userComplete"=>"Generation completed successfully..",
                "systemComplete"=>"Generation completed successfully.. You will now exit Okeanos.",
            _=>{c=true;u}},
        _=>{c=true;u}}),
/*  */  "note" => format!("\x1b[1;2m>>\x1b[0m {}", match identifier[1] {
            "profile" => match identifier[2] {
                "generating"=>"Generating default Okeanos profile..",
            _=>{c=true;u}},
            "privileges" => match identifier[2] {
                "elevated"=>"Successfully elevated privileges.",
                "sufficient"=>"You have sufficient privileges to create a system-wide configuration file.",
                "required"=>"You will be prompted to enter your password in-order to elevate your privileges.",
                "prompt"=>"Press Enter to continue; If the process was successful, Okeanos will restart.",
            _=>{c=true;u}},
        _=>{c=true;u}}),
/*  */  "warning" => format!("\x1b[1;33m!!\x1b[0m {}", match identifier[1] {
            "prompt" => match identifier[2] {
                "doubleQuote"=>"DoubleQuote >> ",
                "singleQuote"=>"SingleQuote >> ",
                "invalidChoice"=>"Invalid choice, only pick `y`, `yes` or `n`, `no`.",
            _=>{c=true;u}},
            "profile" => match identifier[2] {
                "deletionProbability"=>"If this isn't your first time running, your system-wide configurations might be deleted..",
                "systemMissing"=>"System-wide configuration failed to be located. Was it accidentally deleted during this session?",
                "userMissing"=>"User configuration failed to be located. Was it accidentally deleted during this session?",
            _=>{c=true;u}},
        _=>{c=true;u}}),
/*  */  "errors" => format!("\x1b[1;31mxx\x1b[0m {}", match identifier[1] {
            "configurations" => match identifier[2] {
                "sudoElevation"=>"Unexpected error while attempting to elevate privileges.",
                "insufficientPrivileges"=>"You don't have sufficient privileges to create a system-wide configuration file.",
                "missingSystemProfile"=>"Okeanos system profile could not be located.",
                "missingUserProfile"=>"Okeanos user profile could not be located.",
            _=>{c=true;u}},
            "shell" => match identifier[2] {
                "directorySet"=>"Unexpected error while trying to change directory",
                "requiredSingleArgument"=>"command `cd` requires only one argument to be passed",
                "cdArgumentIsNotADirectory"=>"Passed argument was not a directory",
                "pathDoesntExist"=>"Specified path could not be located, i.e. does not exist.",
            _=>{c=true;u}},
            "git" => match identifier[2] {
                "invalidRepoURL"=>"An invalid url was retrived.",
            _=>{c=true;u}},
            "parser" => match identifier[2] {
                "missingFileRedirect"=>"Missing filename to redirect output to after symbol `>`",
                "bufferFetch"=>"Unexpected error while trying to fetch character in buffer.",
            _=>{c=true;u}},
            "noNTSupport"=>"Okeanos does not support Windows NT systems.",
            "unknownArgument"=>"An unknown argument has been passed.",

            "utf8Conversion"=>"Unexpected error while attempting to convert stdout into &str.",
            "stdinFail"=>"Unexpected error while attempting to get user input from stdin.",
            "commandSpawnFail"=>"Unable to spawn child process for command",
            "pathRetrivialFail"=>"Unable to retrive path for prompt.",
            "fileCreation"=>"Unexpected error while attempting to create a file.",
            "fileWrite"=>"Unexpected error while attempting to write to a file.",
            "unwrapFail"=>"Unexpected error while attempting to unwrap.",
        _=>{c=true;u}}),
    _=>n }; if final_.is_empty() || c { eprintln!("No matches found for `{}`", text); exit(-1); } else {
final_ }}