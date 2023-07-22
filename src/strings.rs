use std::collections::HashMap;

const VERSION: &str = "v0.1.0";
pub fn commands() -> HashMap<&'static str, String> { [
    ("help", format!("\x1b[1mOkeanos {}\x1b[0m\nA shell project done \x1b[4mterrible\x1b[0m.\n\nProgram parameters:\n    \x1b[1mOkeanos\x1b[0m <INPUT> [FLAGS]\n\nProgram flags:\n    \x1b[1m--help\x1b[0m & \x1b[1m-h\x1b[0m  ::  Shows Okeanos's description and usage.\n    \x1b[1m--version\x1b[0m & \x1b[1m-v\x1b[0m  ::  Outputs current version of the shell.", VERSION)),
    ("version", VERSION.to_string()),
].iter().cloned().collect::<HashMap<&str, String>>() }


pub fn errors() -> HashMap<&'static str, String> { let mut map = HashMap::new(); for current in [
    ("noNTSupport", "Okeanos does not support Windows NT systems."),
    
    // Command line arguments parser
    ("unknownArgument", "An unknown argument has been passed."),

    // file stuff
    ("fileCreationFail", "Unexpected error while attempting to create a file."),
    ("fileWriteFail", "Unexpected error while attempting to write to a file."),

    // get_git_info()
    ("utf8Conversion", "Unexpected error while attempting to convert stdout into &str."),
    ("invalidRepoUrl", "An invalid url was retrived."),

    // parser.rs
    ("missingFileRedirect", "Missing filename to redirect output to after symbol `>`"),

    // prompt_input()
    ("stdinFail", "Unexpected error while attempting to get user input from stdin."),

    // config_handler.rs
    ("sudoElevateFail", "Unexpected error while attempting to elevate privileges."),
    ("insufficientPrivileges", "You don't have sufficient privileges to create a system-wide configuration file."),
    ("missingSystemProfile", "Okeanos system profile could not be located."),
    ("missingUserProfile", "Okeanos user profile could not be located."),

    // commands.rs
    ("commandSpawnFail", "Unable to spawn child process for command"),

    // Built-in commands
    ("directorySetFail", "Unexpected error while trying to change directory"),
    ("requiredSingleArgument", "command `cd` requires only one argument to be passed"),
    ("cdArgumentIsNotADirectory", "Passed argument was not a directory"),
    ("pathDoesntExist", "Specified path could not be located, i.e. does not exist."),
].iter().cloned().collect::<HashMap<&str, &str>>() { map.insert(current.0, format!("\x1b[1;31mxx\x1b[0m {}", current.1)); } map }


#[allow(unused)] // used for smt that broke my brain
pub fn warnings() -> HashMap<&'static str, String> { let mut map = HashMap::new(); for current in [
    // parser.rs
    ("doubleQuotePrompt", "DoubleQuote >> "),
    ("SingleQuotePrompt", "SingleQuote >> "),

    // config_handler.rs + utility.rs
    ("invalidChoice", "Invalid choice, only pick `y`, `yes` or `n`, `no`."),
    ("profileMaybeDeleted", "If this isn't your first time running, your system-wide configurations might be deleted.."),
    ("missingSysProfile", "System-wide configuration failed to be located. Was it accidentally deleted during this session?"),
    ("missingUserProfile", "User configuration failed to be located. Was it accidentally deleted during this session?"),
].iter().cloned().collect::<HashMap<&str, &str>>() { map.insert(current.0, format!("\x1b[1;33m!!\x1b[0m {}", current.1)); } map }


pub fn information() -> HashMap<&'static str, String> { let mut map = HashMap::new(); for current in [
    // config_handler.rs
    ("noUserProfile", "No local Okeanos profile could be found. Generate one?"),
    ("noSystemProfile", "No system Okeanos profile could be found. Is this your first time running?"),
    ("reqGenerateProfile", "Would you want to generate a new, default, system-wide Okeanos profile?"),
    ("generationCompleteSys", "Generation completed successfully.. You will now exit Okeanos."),
    ("generationCompleteUser", "Generation completed successfully.."),
].iter().cloned().collect::<HashMap<&str, &str>>() { map.insert(current.0, format!("\x1b[1;36m!!\x1b[0m {}", current.1)); } map }


pub fn note() -> HashMap<&'static str, String> { let mut map = HashMap::new(); for current in [
    // config_handler.rs
    ("privilegesElevated", "Successfully elevated privileges."),
    ("sufficientPrivileges", "You have sufficient privileges to create a system-wide configuration file."),
    ("requirePrivileges", "You will be prompted to enter your password in-order to elevate your privileges."),
    ("requirePrompt", "Press Enter to continue; If the process was successful, Okeanos will restart."),
    ("generatingProfile", "Generating default Okeanos profile.."),
].iter().cloned().collect::<HashMap<&str, &str>>() { map.insert(current.0, format!("\x1b[1;2m>>\x1b[0m {}", current.1)); } map }