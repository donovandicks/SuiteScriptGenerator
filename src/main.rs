#[macro_use]
extern crate clap;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

// TODO: Add list of accepted SuiteScript API modules 

const TYPES: [&str; 7] = [
    "MapReduce",
    "UserEvent",
    "Scheduled",
    "Client",
    "Suitelet",
    "Portlet",
    "RESTlet",
];

const API: [&str; 4] = [
    "2.1",
    "2",
    "2.x",
    "2.0",
];

const COPYRIGHT: &str = "/**
 * Copyright (c) 2021 LogMeIn,
 * 320 Summer St, Boston, MA,
 * All Rights Reserved.,
 *,
 * THIS PROGRAM IS CONFIDENTIAL AND PROPRIETARY TO LOGMEIN,
 * AND CONSTITUTES A VALUABLE TRADE SECRET.,
 */

";

fn main() {
    let matches = clap_app!(SuiteScriptGenerator =>
        (version: "0.1.0")
        (author: crate_authors!())
        (about: "Generates a SuiteScript file based on the given inputs")
        (@arg FileName: -f --filename +takes_value +required {validate_file_name} "The name of the JavaScript file to be created")
        (@arg ScriptType: -t --stype +takes_value {validate_script_type} "The type of SuiteScript to create")
        (@arg APIVersion: -v --version +takes_value {validate_api_version} "The SuiteScript API Version to use")
        (@arg Modules: -m --modules +takes_value +multiple "The SuiteScript API modules (N/*) to import into the project")
    ).get_matches();

    let file_name = matches.value_of("FileName").unwrap();
    let mut file = create_file(file_name);

    write_to_file(&mut file, COPYRIGHT);
    write_to_file(&mut file, "/**\n");

    set_script_type(&mut file, &matches);

    let api = matches.value_of("APIVersion").unwrap_or("2.1");
    write_to_file(&mut file, format!(" * @NApiVersion {}\n", api).as_ref());

    write_to_file(&mut file, &" */\n\n");
    write_to_file(&mut file, &"define([\n");

    set_modules(&mut file, &matches);

    write_to_file(&mut file, &"\n});");
}

fn set_script_type(file: &mut File, matches: &clap::ArgMatches) {
    let script_type = matches.value_of("ScriptType");
    if let Some(stype) = script_type {
        match stype {
            "MapReduce" | "UserEvent" | "Scheduled" | "Client" => {
                write_to_file(file, format!(" * @NScriptType {}Script\n", stype).as_ref());
            }
            _ => write_to_file(file, format!(" * @NScriptType {}\n", stype).as_ref()),
        }
    }
}

fn fill_imports(file: &mut File, mods: &[&str]) {
    for module in mods {
        write_to_file(file, format!("  {},\n", module).as_ref());
    }
}

fn fill_amd_args(file: &mut File, mods: &[&str]) {
    for i in 0..mods.len() {
        if i == 0 && i == mods.len() - 1 {
            write_to_file(file, mods[i]);
        } else if i == 0 {
            write_to_file(file, format!("{},", mods[i]).as_ref());
        } else if i == mods.len() - 1 {
            write_to_file(file, format!(" {}", mods[i]).as_ref());
        } else {
            write_to_file(file, format!(" {},", mods[i]).as_ref());
        }
    }
}

fn set_modules(file: &mut File, matches: &clap::ArgMatches) {
    if let Some(modules) = matches.values_of("Modules") {
        let mods: Vec<&str> = modules.collect();
        fill_imports(file, &mods);
        write_to_file(file, &"], (");
        fill_amd_args(file, &mods);
        write_to_file(file, &") => {\n");
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

fn validate_file_name(name: String) -> Result<(), String> {
    let path = Path::new(&name);
    if let Some(ext) = path.extension() {
        if ext != "js" {
            return Err(String::from("Invalid file type"));
        }
    } else {
        return Err(String::from("File name missing extension"));
    }
    Ok(())
}

fn validate_script_type(name: String) -> Result<(), String> {
    if TYPES.contains(&&name[..]) {
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

