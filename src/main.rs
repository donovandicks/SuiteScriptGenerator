#[macro_use]
extern crate clap;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
mod assets;
use assets::netsuite_types::{TYPES, API, MODULES};

/// Entry point for the CLI.
///
/// Initializes the Clap application. If input validation is successful, creates the file and
/// populates it with the given inputs.
fn main() {
    let matches = init_app();
    let file_name = matches.value_of("FileName").unwrap();
    let mut file = create_file(file_name);

    let api = matches.value_of("APIVersion").unwrap_or("2.1");
    
    let contents = format!(
        "{}/**\n{} * @NApiVersion {}\n */\n\ndefine([\n{}\n}});",
        get_copyright(&matches),
        get_script_type(&matches),
        api,
        get_modules(&matches),
    );

    write_to_file(&mut file, contents.as_ref());
}

/// Initializes the CLI application
///
/// The CLI is started with several options. The default configuration is to generate a file with
/// no copyright, no imports, no script type, and with API version 2.1. The script file name is
/// required. Running the command with `-h` or `--help` will print the options and their help
/// information.
fn init_app() -> clap::ArgMatches<'static> {
    clap_app!(SuiteScriptGenerator =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg FileName: -f --filename +takes_value +required {validate_file_name} "The name of the JavaScript file to be created")
        (@arg ScriptType: -t --stype +takes_value {validate_script_type} "The type of SuiteScript to create")
        (@arg APIVersion: -v --version +takes_value {validate_api_version} "The SuiteScript API Version to use")
        (@arg Modules: -m --modules +takes_value +multiple {validate_modules} "The SuiteScript API modules (N/*) to import into the project")
        (@arg CopyrightFile: -c --copyright +takes_value {validate_copyright_file} "A text file containing a copyright doc comment")
    ).get_matches()
}

/// Retrieves the contents of a specified copyright file.
///
/// Reads the specified file into memory. The contents are trimmed to remove any mistaken
/// whitespaces or newlines in the file. The contents are then returned, formatted with one blank
/// line after the final content line of the copyright message. Returns an empty string if no file
/// is specified.
///
/// # Panics
/// The function panics if the file cannot be read
fn get_copyright(matches: &clap::ArgMatches) -> String {
    if let Some(copyright_file) = matches.value_of("CopyrightFile") {
        let contents = std::fs::read_to_string(copyright_file).expect("Failed to read file").trim().to_string();
        return format!("{}\n\n", contents);
    }

    String::from("")
}

/// Converts a given script type name to its supported NetSuite name.
///
/// Converts the name to lowercase to support mangled inputs. Matches the name to the casing
/// supported by NetSuite. If no match, an empty string is returned.
fn map_script_to_name(stype: &str) -> &str {
    match stype.to_lowercase().as_ref() {
        "mapreduce" => "MapReduce",
        "userevent" => "UserEvet",
        "scheduled" => "Scheduled",
        "client" => "Client",
        "suitelet" => "Suitelet",
        "restlet" => "RESTlet",
        "portlet" => "Portlet",
        _ => "",
    }
}

/// Converts a given script type to its supported NetSuite name.
///
/// Checks the clap args for ScriptType. Retrieves either a valid script name or an empty string.
/// If the script name is valid, returns a string with the NScriptType tag and the script name.
/// Otherwise, returns an empty string.
fn get_script_type(matches: &clap::ArgMatches) -> String {
    let script_type = map_script_to_name(matches.value_of("ScriptType").unwrap_or(""));
    match script_type {
        "MapReduce" | "UserEvent" | "Scheduled" | "Client" => format!(" * @NScriptType {}Script\n", script_type),
        "" => String::from(""),
        _ => format!(" * @NScriptType {}\n", script_type),
    }
}

/// Maps a given module name to the valid NetSuite name.
///
/// Converts the module name to lowercase to support mangled inputs. Matches the name to a list of
/// special cases, or returns the lowercase name if no case applies.
fn map_module_to_name(module: &str) -> String {
    let lower_case = module.to_lowercase();
    match lower_case.as_str() {
        "certificatecontrol" => "certificateControl".into(),
        "currentrecord" => "currentRecord".into(),
        "keycontrol" => "keyControl".into(),
        "recordcontext" => "recordContext".into(),
        "suiteappinfo" => "suiteAppInfo".into(),
        "serverwidget" => "serverWidget".into(),
        _ => lower_case,
    }
}

/// Converts a given module name to its supported NetSuite name.
///
/// Maps over a vector of module names, applying `map_module_to_name` to each name.
fn get_module_names(modules: clap::Values) -> Vec<String> {
    let mods: Vec<&str> = modules.collect();
    mods.iter().map(|name| map_module_to_name(name)).collect()
}

/// Formats a list of NetSuite module names into the correct import string.
///
/// Joins modules with a comma, newline, and prefix of `N/`. Indentation is 2 spaces.
fn format_imports(modules: &Vec<String>) -> String {
    modules.join("',\n  'N/")
}

/// Formats a list of NetSuite module names into an argument list.
///
/// Joins modules with a comma and space.
fn format_args(modules: &Vec<String>) -> String {
    modules.join(", ")
}

/// Writes the given SuiteScript import modules to the file.
///
/// Checks the clap args for Modules. Returns a string with the formatted imports and args and the
/// symbols around them if modules were passed in. Otherwise, returns a string with the symbols for 
/// an AMD module with no imports.
fn get_modules(matches: &clap::ArgMatches) -> String {
    if let Some(modules) = matches.values_of("Modules") {
        let mods = get_module_names(modules);
        return format!("  'N/{}',\n], ({}) => {{\n", format_imports(&mods), format_args(&mods));
    } else {
        return String::from("], () => {\n");
    }
}

/// Creates a file with a given name.
fn create_file(file_name: &str) -> File {
    File::create(file_name).unwrap()
}

/// Writes given contents to a given file.
fn write_to_file(file: &mut File, contents: &str) {
    file.write_all(contents.as_bytes()).unwrap();
}

/// Checks if a file has an extension.
///
/// Retrieves the file extension from a given path, if available. Otherwise, returns a message that
/// the path does not have an extension.
///
/// # Panics
///
/// Panics if the path extension is not valid unicode.
fn validate_file(path: &Path) -> &str {
    if let Some(ext) = path.extension() {
        ext.to_str().unwrap()
    } else {
        "File name missing extension"
    }
}

/// Validates a given file name for a copyright file.
///
/// A copyright file is required to be a text file. It is assumed that the contents of the file
/// contain a JSDoc style doc comment with a copyright message.
fn validate_copyright_file(name: String) -> Result<(), String> {
    let path = Path::new(&name);
    let ext = validate_file(path);
    if ext != "txt" {
        return Err(String::from("Invalid file type: copyright file must be a text file."));
    }

    Ok(())
}

/// Validates a given file name for a SuiteScript file.
///
/// The file name is checked for its extension and existing parent directories if applicable.
/// SuiteScript files must have a `.js` extension.
fn validate_file_name(name: String) -> Result<(), String> {
    let path = Path::new(&name);
    let ext = validate_file(path);
    if ext != "js" {
        return Err(String::from("Invalid file type: SuiteScript file must be a JavaScript file."));
    }

    if name.contains('/') || name.contains('\\') {
        if let Some(parent) = path.parent() {
            if !parent.is_dir() {
                return Err(String::from("Parent directory does not exist"));
            }
        }
    }

    Ok(())
}

/// Validates a given SuiteScript script type against the list of supported script types.
///
/// Converts the given script name to lowercase to support mangled inputs. Checks the lowercase
/// name against the list of supported script types in `assets/`.
fn validate_script_type(name: String) -> Result<(), String> {
    let lower_case = name.to_lowercase();
    if TYPES.contains(&&lower_case[..]) {
        return Ok(());
    }

    Err(String::from("Invalid script type"))
}

/// Validates a given SuiteScript API version against the list of supported versions.
fn validate_api_version(api: String) -> Result<(), String> {
    if API.contains(&&api[..]) {
        return Ok(());
    }

    Err(String::from("Invalid API version"))
}

/// Validates a given NetSuite module name against the list of supported modules.
///
/// Converts the given module to lowercase to support mangled inputs. Checks the lowercase name
/// against the list of supported modules in `assets/`.
fn validate_modules(name: String) -> Result<(), String> {
    let lower_case = name.to_lowercase();
    if !MODULES.contains(&&lower_case[..]) {
        return Err(format!("Invalid module name {}", name))
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mod() -> Result<(), String> {
        assert_eq!(validate_modules(String::from("record")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_mod() -> Result<(), String> {
        assert_eq!(validate_modules(String::from("reecord")), Err(String::from("Invalid module name reecord")));
        Ok(())
    }

    #[test]
    fn test_valid_api() -> Result<(), String> {
        assert_eq!(validate_api_version(String::from("2")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_api() -> Result<(), String> {
        assert_eq!(validate_api_version(String::from("1")), Err(String::from("Invalid API version")));
        Ok(())
    }

    #[test]
    fn test_valid_script_type() -> Result<(), String> {
        assert_eq!(validate_script_type(String::from("mapreduce")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_script_type() -> Result<(), String> {
        assert_eq!(validate_script_type(String::from("rest")), Err(String::from("Invalid script type")));
        Ok(())
    }

    #[test]
    fn test_valid_file() -> Result<(), String> {
        assert_eq!(validate_file(Path::new("test.js")), "js");
        Ok(())
    }

    #[test]
    fn test_invalid_file() -> Result<(), String> {
        assert_eq!(validate_file(Path::new("test")), "File name missing extension");
        Ok(())
    }

    #[test]
    fn test_valid_copyright() -> Result<(), String> {
        assert_eq!(validate_copyright_file(String::from("copyright.txt")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_copyright() -> Result<(), String> {
        assert_eq!(validate_copyright_file(String::from("copyright")),
            Err(String::from("Invalid file type: copyright file must be a text file.")));
        Ok(())
    }

    #[test]
    fn test_valid_script_file() -> Result<(), String> {
        assert_eq!(validate_file_name(String::from("test.js")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_script_file() -> Result<(), String> {
        assert_eq!(validate_file_name(String::from("test")),
            Err(String::from("Invalid file type: SuiteScript file must be a JavaScript file.")));
        Ok(())
    }

    #[test]
    fn test_valid_script_parent_dir() -> Result<(), String> {
        assert_eq!(validate_file_name(String::from("src/test.js")), Ok(()));
        Ok(())
    }

    #[test]
    fn test_invalid_script_parent_dir() -> Result<(), String> {
        assert_eq!(validate_file_name(String::from("nonexistent/test.js")), 
            Err(String::from("Parent directory does not exist")));
        Ok(())
    }
}
