use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
mod assets;
use assets::netsuite_types::{API, MODULES, TYPES};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "suitescript",
    about = "CLI to create SuiteScript files and generate boilerplate"
)]
struct Opt {
    /// Name of the file to be generated
    #[structopt(short, long = "filename", parse(from_os_str), validator = validate_file_name)]
    file_name: PathBuf,

    /// Type of `SuiteScript` to be generated
    #[structopt(short, long = "scripttype", default_value = "", validator = validate_script_type)]
    script_type: String,

    /// Version of the `SuiteScript` API to use
    #[structopt(short, long = "apiversion", default_value = "2.1", validator = validate_api_version)]
    api_version: String,

    /// `SuiteScript` modules to import
    #[structopt(short, long = "modules", default_value = "", validator = validate_modules)]
    modules: Vec<String>,

    /// Path to a file containing your company's copyright message
    #[structopt(short, long = "copyright", parse(from_os_str), default_value = "", validator = validate_copyright_file)]
    copyright: PathBuf,
}

/// Entry point for the CLI.
///
/// Initializes the application. If input validation is successful, creates the file and
/// populates it according to the given inputs.
fn main() {
    let config = Opt::from_args();
    let mut file = create_file(&config.file_name);

    let contents = format!(
        "{}/**\n{} * @NApiVersion {}\n */\n\ndefine([\n{}\n}});",
        get_copyright(&config.copyright),
        get_script_type(config.script_type.as_ref()),
        get_api_version(config.api_version.as_ref()),
        get_modules(&config.modules),
    );

    write_to_file(&mut file, contents.as_ref());
}

/// Gets the `SuiteScript` API version to be used.
fn get_api_version(version: &str) -> String {
    match version {
        "2" => String::from("2.0"),
        _ => version.to_owned(),
    }
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
fn get_copyright(copyright: &Path) -> String {
    if copyright.to_str().unwrap() == "" {
        return String::from("");
    }

    let contents = std::fs::read_to_string(copyright)
        .expect("Failed to read file")
        .trim()
        .to_string();
    format!("{}\n\n", contents)
}

/// Converts a given script type name to its supported `NetSuite` name.
///
/// Converts the name to lowercase to support mangled inputs. Matches the name to the casing
/// supported by `NetSuite`. If no match, an empty string is returned.
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

/// Converts a given script type to its supported `NetSuite` name.
///
/// If the script name is valid, returns a string with the NScriptType tag and the script name.
/// Otherwise, returns an empty string.
fn get_script_type(script_type: &str) -> String {
    let script_name = map_script_to_name(script_type);
    match script_name {
        "MapReduce" | "UserEvent" | "Scheduled" | "Client" => {
            format!(" * @NScriptType {}Script\n", script_name)
        }
        "" => String::from(""),
        _ => format!(" * @NScriptType {}\n", script_type),
    }
}

/// Maps a given module name to the valid `NetSuite` name.
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

/// Converts a given module name to its supported `NetSuite` name.
///
/// Maps over a vector of module names, applying `map_module_to_name` to each name.
fn get_module_names(modules: &[String]) -> Vec<String> {
    modules
        .iter()
        .map(|name| map_module_to_name(name))
        .collect()
}

/// Formats a list of `NetSuite` module names into the correct import string.
///
/// Joins modules with a comma, newline, and prefix of `N/`. Indentation is 2 spaces.
fn format_imports(modules: &[String]) -> String {
    modules.join("',\n  'N/")
}

/// Formats a list of `NetSuite` module names into an argument list.
///
/// Removes any `/` in module names. Joins modules with a comma and space.
fn format_args(modules: &[String]) -> String {
    let cleaned: Vec<String> = modules.iter().map(|name| name.replace('/', "")).collect();
    cleaned.join(", ")
}

/// Writes the given `SuiteScript` import modules to the file.
///
/// Returns a string with the formatted imports and args and the symbols around them if modules
/// were passed in. Otherwise, returns a string with the symbols for an AMD module with no imports.
fn get_modules(modules: &[String]) -> String {
    if modules == vec![String::from("")] {
        return String::from("], () => {\n");
    }

    let mods = get_module_names(modules);
    format!(
        "  'N/{}',\n], ({}) => {{\n",
        format_imports(&mods),
        format_args(&mods)
    )
}

/// Creates a file with a given name.
fn create_file(file_name: &Path) -> File {
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
    path.extension()
        .map_or("File name missing extension", |ext| ext.to_str().unwrap())
}

/// Validates a given file name for a copyright file.
///
/// A copyright file is required to be a text file. It is assumed that the contents of the file
/// contain a JSDoc style doc comment with a copyright message.
fn validate_copyright_file(name: String) -> Result<(), String> {
    // TODO: Check if file exists
    if name.is_empty() {
        return Ok(());
    }

    let path = Path::new(&name);
    let ext = validate_file(path);
    if ext != "txt" {
        return Err(String::from(
            "Invalid file type: copyright file must be a text file.",
        ));
    }

    Ok(())
}

/// Validates a given file name for a `SuiteScript` file.
///
/// The file name is checked for its extension and existing parent directories if applicable.
/// SuiteScript files must have a `.js` extension.
fn validate_file_name(name: String) -> Result<(), String> {
    let path = Path::new(&name);
    let ext = validate_file(path);
    if ext != "js" {
        return Err(String::from(
            "Invalid file type: SuiteScript file must be a JavaScript file.",
        ));
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

/// Validates a given `SuiteScript` script type against the list of supported script types.
///
/// Converts the given script name to lowercase to support mangled inputs. Checks the lowercase
/// name against the list of supported script types in `assets/`.
fn validate_script_type(name: String) -> Result<(), String> {
    if name.is_empty() {
        return Ok(());
    }

    let lower_case = name.to_lowercase();
    if TYPES.contains(&&lower_case[..]) {
        return Ok(());
    }

    Err(String::from("Invalid script type"))
}

/// Validates a given `SuiteScript` API version against the list of supported versions.
fn validate_api_version(api: String) -> Result<(), String> {
    if API.contains(&&api[..]) {
        return Ok(());
    }

    Err(String::from("Invalid API version"))
}

/// Validates a given `NetSuite` module name against the list of supported modules.
///
/// Converts the given module to lowercase to support mangled inputs. Checks the lowercase name
/// against the list of supported modules in `assets/`.
fn validate_modules(name: String) -> Result<(), String> {
    if name.is_empty() {
        return Ok(());
    }

    let lower_case = name.to_lowercase();
    if !MODULES.contains(&&lower_case[..]) {
        return Err(format!("Invalid module name {}", name));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mod() {
        assert_eq!(validate_modules(String::from("record")), Ok(()));
    }

    #[test]
    fn test_invalid_mod() {
        assert_eq!(
            validate_modules(String::from("reecord")),
            Err(String::from("Invalid module name reecord"))
        );
    }

    #[test]
    fn test_valid_api() {
        assert_eq!(validate_api_version(String::from("2")), Ok(()));
    }

    #[test]
    fn test_invalid_api() {
        assert_eq!(
            validate_api_version(String::from("1")),
            Err(String::from("Invalid API version"))
        );
    }

    #[test]
    fn test_valid_script_type() {
        assert_eq!(validate_script_type(String::from("mapreduce")), Ok(()));
    }

    #[test]
    fn test_invalid_script_type() {
        assert_eq!(
            validate_script_type(String::from("rest")),
            Err(String::from("Invalid script type"))
        );
    }

    #[test]
    fn test_valid_file() {
        assert_eq!(validate_file(Path::new("test.js")), "js");
    }

    #[test]
    fn test_invalid_file() {
        assert_eq!(
            validate_file(Path::new("test")),
            "File name missing extension"
        );
    }

    #[test]
    fn test_valid_copyright() {
        assert_eq!(
            validate_copyright_file(String::from("copyright.txt")),
            Ok(())
        );
    }

    #[test]
    fn test_invalid_copyright() {
        assert_eq!(
            validate_copyright_file(String::from("copyright")),
            Err(String::from(
                "Invalid file type: copyright file must be a text file."
            ))
        );
    }

    #[test]
    fn test_valid_script_file() {
        assert_eq!(validate_file_name(String::from("test.js")), Ok(()));
    }

    #[test]
    fn test_invalid_script_file() {
        assert_eq!(
            validate_file_name(String::from("test")),
            Err(String::from(
                "Invalid file type: SuiteScript file must be a JavaScript file."
            ))
        );
    }

    #[test]
    fn test_valid_script_parent_dir() {
        assert_eq!(validate_file_name(String::from("src/test.js")), Ok(()));
    }

    #[test]
    fn test_invalid_script_parent_dir() {
        assert_eq!(
            validate_file_name(String::from("nonexistent/test.js")),
            Err(String::from("Parent directory does not exist"))
        );
    }

    #[test]
    fn test_format_imports() {
        assert_eq!(
            format_imports(&vec!["record".into(), "search".into()]),
            String::from("record',\n  'N/search")
        )
    }

    #[test]
    fn test_format_args() {
        assert_eq!(
            format_args(&vec!["record".into(), "search".into(), "ui/dialog".into()]),
            String::from("record, search, uidialog")
        )
    }

    #[test]
    fn test_get_mod_names() {
        assert_eq!(
            get_module_names(&vec![String::from("rEcOrD"), String::from("RECORDcontext")]),
            vec![String::from("record"), String::from("recordContext")]
        )
    }

    #[test]
    fn test_map_script_name() {
        assert_eq!(map_script_to_name("mApReDuCe"), "MapReduce")
    }
}
