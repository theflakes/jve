extern crate serde_json;
extern crate itertools;

use std::io;
use std::{env, process};
use serde_json::{Value};
use std::collections::HashSet;
use itertools::Itertools;


fn print_results(output: &Vec<String>, split_fields: Vec<&str>, delim: &str) {
    let mut results = String::new();
    match delim {
        "\\n" => {
                    split_fields.iter().zip(output.iter())
                        .map(|(a, b)| format!("{}: {}", a, b))
                        .for_each(|o| println!("[*] {}", o));
                    println!();
                    return
                },
        "\\t" => results = output.join("\t"),
        _     => results = output.join(delim)
    }
    // eh, ugly but tired now
    if !results.is_empty() && !results.eq("") 
        && !results.eq("\"\"") && !results.eq("[\"\"]")
            { println!("{}", results); }
}

fn string_to_json(input: String) -> io::Result<Value> {
    let json: Value = {
        let this = serde_json::from_str(&input);
        match this {
            Ok(t) => t,
            Err(_e) => {
                return Ok(Value::Null);
            },
        }
    };
    Ok(json)
}

fn get_first_elem(set: &HashSet<String>) -> Option<&String> {
    set.iter().next()
}

// If targeted key is an array, concat it using comma delim
fn join_values(array: &HashSet<String>) -> String {
    let mut output = String::new();
    if array.is_empty() { return output }
    if array.len() == 1 { 
        output = match get_first_elem(array) {
            Some(o) => o.into(),
            _ => String::new()
        };
        return output
    }
    return format!("\"{}\"", array.iter().join(","))
}

// Setup for havesting targeted key's values
fn get_key_values(json: &Value, field_paths: &str, delim: &str) {
    let paths: Vec<&str> = field_paths.split(",").collect();
    let mut values = Vec::new();
    for path in paths.iter() {
        let field_names: Vec<&str> = path.split('.').collect();
        let mut results: HashSet<String> = HashSet::new();
        traverse_json_value(json, &field_names, &mut results);
        values.push(join_values(&results));
    }
    print_results(&values, paths, delim);
}

/*
   Recursively traverse Json structure to build array of values found in a key across all logs
*/
fn traverse_json_value(json: &Value, field_names: &[&str], values: &mut HashSet<String>) {
    if let Some((first_field_name, remaining_field_names)) = field_names.split_first() {
        match json {
            Value::Object(map) => {
                if let Some(value) = map.get(*first_field_name) {
                    if remaining_field_names.is_empty() {
                        values.insert(value.to_string());
                    } else {
                        traverse_json_value(value, remaining_field_names, values);
                    }
                }
            }
            Value::Array(vec) => {
                for value in vec {
                    traverse_json_value(value, field_names, values);
                }
            }
            _ => {}
        }
    }
}

fn print_uniques(mut uniques: &HashSet<String>) {
    let mut values: Vec<String> = uniques.into_iter().cloned().collect();
    values.sort();
    for v in values { println!("{}", v) }
}

// If not using new line delim, print field header
fn print_header(fields: &str, delim: &str) {
    if delim.eq("\\n") { return; }
    match delim {
        "\\t" => println!("{}", fields.replace(",", "\t")),
        _ => println!("{}", fields.replace(",", &delim))
    }
}

// Build dot delimited key paths as we traverse the Json structure
fn get_new_prefix(prefix: &str, key: &str) -> String{
    let new_prefix = if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{}.{}", prefix, key)
    };
    return new_prefix
}

/*
   Recursively traverse Json structure to build dot delimited key paths
*/
fn traverse_json(json: &Value, prefix: &str, paths: &mut HashSet<String>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_prefix = get_new_prefix(&prefix, key);
                traverse_json(value, &new_prefix, paths);
            }
        }
        Value::Array(vec) => {
            if let Some(first_element) = vec.first() {
                traverse_json(first_element, prefix.clone(), paths);
            }
            paths.insert(prefix.to_string());
        }
        _ => {
            paths.insert(prefix.to_string());
        }
    }
}

fn process_uniques(
                    keys: &Vec<&str>, 
                    get_values: bool, 
                    log: &Value,
                    value: &str
                ) -> HashSet<String> {
    let mut uniques: HashSet<String> = HashSet::new();
    // Get all field names across all logs
    if keys.is_empty() {
        traverse_json(log, &"".to_string(), &mut uniques);
    } else {
        if !check_key_value(&log, &keys, &value) { return uniques; }
        // get all uniqued values of a given field
        if get_values {
            traverse_json_value(&log, &keys, &mut uniques);
        // get all uniqued field names where a given field exists in the log
        } else {
            traverse_json(log, &"".to_string(), &mut uniques);
        }
    }
    return uniques
}

fn found_in_vec(values: &HashSet<String>, value: &str) -> bool {
    for u in values {
        if u.to_lowercase().contains(value) {
            return true
        }
    }
    return false
}

// Does the dot delimited Json key path exists?
fn path_exists(json: &Value, keys: &[&str]) -> bool {
    if let Some((first_key, remaining_keys)) = keys.split_first() {
        match json {
            Value::Object(map) => {
                if let Some(value) = map.get(*first_key) {
                    if remaining_keys.is_empty() {
                        return true
                    } else {
                        path_exists(value, remaining_keys)
                    }
                } else {
                    return false
                }
            }
            Value::Array(array) => {
                if let Some(first_element) = array.first() {
                    path_exists(first_element, keys)
                } else {
                    return false
                }
            }
            _ => return false,
        }
    } else {
        return false
    }
}

// Verify Key Value pair exist
fn check_key_value(log: &Value, keys: &Vec<&str>, value: &str) -> bool {
    if value.is_empty() { return path_exists(log, &keys) }
    let mut values: HashSet<String> = HashSet::new();
    traverse_json_value(log, &keys, &mut values);
    if values.is_empty() { return false; }
    return found_in_vec(&values, &value)
}

fn main() -> io::Result<()> {
    let (
        fields, 
        delim, 
        get_uniques, 
        field_name,
        get_values,
        value
    ) = get_args()?;

    let stdin = io::stdin();

    if !get_uniques { print_header(&fields, &delim) };
    
    let mut uniques = HashSet::new();
    let no_whitespace = field_name.replace(char::is_whitespace, "");
    let mut keys: Vec<&str> = no_whitespace.split('.').collect();
    keys.retain(|&k| k != "");

    for line in stdin.lines() {
        let l = match line {
            Ok(o) => o,
            Err(_) => {continue},
        };

        let log = string_to_json(l)?;

        // We are only looking for unique field names or values in a given field
        if get_uniques {
            uniques.extend(process_uniques(&keys, get_values, &log, &value));
            continue;
        }

        // only print out logs where the given key exists  
        // and/or its value contains specified value
        if !keys.is_empty() {
            if !check_key_value(&log, &keys, &value) { continue; }
            get_key_values(&log, &fields, &delim);
            continue;
        }

        // we just want all fields specified 
        // including null results from log not having those fields
        get_key_values(&log, &fields, &delim);
    }

    // if we were only looking for uniques, print what was found
    if get_uniques { print_uniques(&uniques) }
    Ok(())
}

fn get_args() -> io::Result<(String, String, bool, String, bool, String)> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { print_help(); }
    let mut fields = String::new();
    let mut delim = String::new();
    let mut get_fields = false;
    let mut get_delim = false;
    let mut get_uniques = false;
    let mut get_values = false;
    let mut get_key = false;
    let mut key = String::new();
    let mut get_string = false;
    let mut string = String::new();
    for arg in args {
        match arg.as_str() {
            "-f" | "--fields" => get_fields = true,
            "-d" | "--delimiter" => get_delim = true,
            "-k" | "--key" => get_key = true,
            "-s" | "--string" => get_string = true,
            "-u" | "--unique" => get_uniques = true,
            "-v" | "--values" => get_values = true,
            _ => {
                if get_fields {
                    fields = arg.to_string();
                    get_fields = false;
                } else if get_delim {
                    delim = arg.to_string();
                    get_delim = false;
                } else if get_key {
                    key = arg.to_string();
                    get_key = false;
                } else if get_string {
                    string = arg.to_string();
                    get_string = false;
                }
            }
        }
    }
    if fields.is_empty() ^ delim.is_empty() {
        println!("If either '--delimiter' or '--fields' is used, both must be used.");
        print_help();
    }
    if !string.is_empty() && key.is_empty() {
        println!("If '--string' is used then '--key' must be used.");
        print_help();
    }
    Ok((fields, delim, get_uniques, key, get_values, string.to_lowercase()))
}


fn print_help() {
    let help = "
Author: Brian Kellogg
License: MIT
Purpose: Extract json field values

JVE - Json Value Extractor

This program accepts piping line delimited json input via output from some previous command.

Usage: 
    cat logs.json | jve --delimiter ',' --fields 'filename,hashes.md5,hashes.ssdeep'
        - comma seperated output
    cat logs.json | jve -d '\\n' -f 'filename,hashes.md5,hashes.ssdeep'
        - output to a new line for each field
    cat logs.json | jve -d '\\t' -f 'filename,hashes.md5,hashes.ssdeep'
        - tab seperated output
    cat logs.json | jve -d ',' -f 'filename,hashes.md5' --key 'path'
        - comma seperated list of all fields only where the key named 'path' exists
    cat logs.json | jve -d ',' -f 'filename,hashes.md5' -k 'path' --string '/home/evil'
        - comma seperated list of all fields only where the key named 'path' exists
          and the 'path' key's value contains the string '/home/evil'
    cat logs.json | jve --unique
        - Collect and print a uniqued list of all key names found in all logs
        - Nested key names will be dot delimited
    cat logs.json | jve --unique --key 'key_name'
        - Collect and print a uniqued list of all key names found in logs with 
          the specified 'key_name'
    cat logs.json | jve --unique --values --key 'key_name'
        - print a uniqued list of all values found in the key 'key_name' 
          across all logs

Options:
    -d, --delimiter ','             Value to use to seperate key value output
                                    - when using a new line delimiter, array values
                                      will be comma seperated
    -f, --fields 'a.b.c.d,a.b.e'    Comma seperated list of keys in dot notation
    -k, --key 'name_of_key'         Only examine logs where the specified key exists
    -s, --string 'string'           Only examine logs where the specified key's value
                                    contains the specified string
                                    - must be used with '--key'
                                    - case insensitive match
    -u, --unique                    Get uniqued entries for: 
                                    - if used by itself, all field names across 
                                      all logs
                                    - unique key names of logs wherein the given 
                                      key exists
                                    - if '--values' is also specified, list all the
                                      unique values of the specified key '--key'
                                    - Nested key names will be dot delimited
    -v, --values                    Must be used along with '--unique' and '--key'
                                    - print the unique values of the specified key

NOTE:   If a key is an array or the key name occurs in an array, 
        this program will concatenate all array key values into a 
        delimited quoted string across all array elements.
";
    println!("{}", help);
    process::exit(1)   
}