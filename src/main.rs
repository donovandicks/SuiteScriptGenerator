#[macro_use]
extern crate clap;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
mod assets;
use assets::netsuite_types::{TYPES, API, MODULES};

fn main() {
    let matches = init_app();
    let file_name = matches.value_of("FileName").unwrap();
    let mut file = create_file(file_name);

    write_to_file(&mut file, format!("{}/**\n{}", get_copyright(&matches), get_script_type(&matches)).as_ref());

    let api = matches.value_of("APIVersion").unwrap_or("2.1");
    write_to_file(&mut file, format!(" * @NApiVersion {}\n */\n\ndefine([\n", api).as_ref());

    write_modules(&mut file, &matches);

    write_to_file(&mut file, &"\n});");
}

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

fn get_copyright(matches: &clap::ArgMatches) -> String {
    if let Some(copyright_file) = matches.value_of("CopyrightFile") {
        let contents = std::fs::read_to_string(copyright_file).expect("Failed to read file").trim().to_string();
        return format!("{}\n\n", contents);
    }

    String::from("")
}

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

fn get_script_type(matches: &clap::ArgMatches) -> String {
    let script_type = map_script_to_name(matches.value_of("ScriptType").unwrap_or(""));
    match script_type {
        "MapReduce" | "UserEvent" | "Scheduled" | "Client" => format!(" * @NScriptType {}Script\n", script_type),
        "" => String::from(""),
        _ => format!(" * @NScriptType {}\n", script_type),
    }
}

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

fn get_module_names(modules: clap::Values) -> Vec<String> {
    let mods: Vec<&str> = modules.collect();
    mods.iter().map(|name| map_module_to_name(name)).collect()
}

fn write_modules(file: &mut File, matches: &clap::ArgMatches) {
    if let Some(modules) = matches.values_of("Modules") {
        let mods = get_module_names(modules);
        let imports = mods.join("',\n  'N/");
        let args = mods.join(", ");
        write_to_file(file, format!("  'N/{}',\n], ({}) => {{\n", imports, args).as_ref());
    } else {
        write_to_file(file, &"], () => {\n");
    }
}

fn create_file(file_name: &str) -> File {
    File::create(file_name).unwrap()
}

fn write_to_file(file: &mut File, contents: &str) {
    file.write_all(contents.as_bytes()).unwrap();
}

fn validate_file(path: &Path) -> &str {
    if let Some(ext) = path.extension() {
        ext.to_str().unwrap()
    } else {
        "File name missing extension"
    }
}

fn validate_copyright_file(name: String) -> Result<(), String> {
    let path = Path::new(&name);
    let ext = validate_file(path);
    if ext != "txt" {
        return Err(String::from("Invalid file type: copyright file must be a text file."));
    }

    Ok(())
}

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

fn validate_script_type(name: String) -> Result<(), String> {
    let lower_case = name.to_lowercase();
    if TYPES.contains(&&lower_case[..]) {
        return Ok(());
    }

    Err(String::from("Invalid script type"))
}

fn validate_api_version(api: String) -> Result<(), String> {
    if API.contains(&&api[..]) {
        return Ok(());
    }

    Err(String::from("Invalid API version"))
}

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
}
