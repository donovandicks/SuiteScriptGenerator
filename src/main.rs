#[macro_use]
extern crate clap;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

// TODO: Add list of accepted SuiteScript API modules 

const TYPES: [&str; 7] = [
    "mapreduce",
    "userevent",
    "scheduled",
    "client",
    "suitelet",
    "portlet",
    "restlet",
];

const API: [&str; 4] = [
    "2.1",
    "2",
    "2.x",
    "2.0",
];

fn main() {
    let matches = init_app();
    let file_name = matches.value_of("FileName").unwrap();
    let mut file = create_file(file_name);

    write_to_file(&mut file, format!("{}/**\n{}", get_copyright(&matches), get_script_type(&matches)).as_ref());

    let api = matches.value_of("APIVersion").unwrap_or("2.1");
    write_to_file(&mut file, format!(" * @NApiVersion {}\n */\n\ndefine([\n", api).as_ref());

    set_modules(&mut file, &matches);

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
        (@arg Modules: -m --modules +takes_value +multiple "The SuiteScript API modules (N/*) to import into the project")
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
        _ => "INVALID",
    }
}

fn get_script_type(matches: &clap::ArgMatches) -> String {
    let script_type = map_script_to_name(matches.value_of("ScriptType").unwrap_or(""));
    match script_type {
        "MapReduce" | "UserEvent" | "Scheduled" | "Client" => {
            return format!(" * @NScriptType {}Script\n", script_type);
        }
        "" => return String::from(""),
        _ => return format!(" * @NScriptType {}\n", script_type),
    }
}

fn set_modules(file: &mut File, matches: &clap::ArgMatches) {
    if let Some(modules) = matches.values_of("Modules") {
        let mods: Vec<&str> = modules.collect();
        let imports = mods.join(",\n  ");
        let args = mods.join(", ");
        write_to_file(file, format!("  {},\n], ({}) => {{\n", imports, args).as_ref());
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

    if name.contains("/") || name.contains("\\") {
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

