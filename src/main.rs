/*
    Ugh, not experienced in Rust enough to code this well. I am sorry.
    But, it seems to work!
*/

extern crate serde_json;
extern crate itertools;

use std::io;
use std::{env, process};
use serde_json::Value;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use colored::Colorize;


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


/*
   Recursively traverse Json structure to build array of values found in a key across all logs
*/
fn traverse_json_values_unique(json: &Value, field_names: &[&str], uniques: &mut HashMap<String, u64>) {
    if let Some((first_field_name, remaining_field_names)) = field_names.split_first() {
        match json {
            Value::Object(map) => {
                if let Some(value) = map.get(*first_field_name) {
                    if remaining_field_names.is_empty() {
                        *uniques.entry(value.to_string()).or_insert(0) += 1;
                    } else {
                        traverse_json_values_unique(value, remaining_field_names, uniques);
                    }
                }
            }
            Value::Array(vec) => {
                for value in vec {
                    traverse_json_values_unique(value, field_names, uniques);
                }
            }
            _ => {}
        }
    }
}

fn print_unique_values(mut uniques: &HashMap<String, u64>, key_sort: bool) {
    let mut u: Vec<(&String, &u64)> = uniques.iter().collect();
    if key_sort {
        // sort by the key based upon whether its a string or integer
        u.sort_by(|a, b| {
            let a_key = a.0.parse::<u64>();
            let b_key = b.0.parse::<u64>();
            if let (Ok(a_key), Ok(b_key)) = (a_key, b_key) {
                a_key.cmp(&b_key)
            } else {
                a.0.cmp(b.0)
            }
        });
    } else {
        u.sort_by(|a, b| a.1.cmp(b.1));
    }
    for (k, v) in u { println!("{}: {}", k, v) }
}

fn print_unique_keys(mut uniques: &HashMap<String, HashSet<String>>) {
    for key in uniques.keys().sorted() {
        let v = uniques[key].clone();
        let mut values: Vec<String> = v.into_iter().collect();
        values.sort();
        match values.len() {
            1 => println!("{}: {}", key, values.join(", ").green()),
            2 => {
                    if values[0] == "array" {
                        let output = format!("{}", values.join("[").to_string() + "]");
                        println!("{}: {}", key, output.green());
                        continue;
                    }
                    println!("{}: {}", key, values.join(", ").red())
                },
            _ => println!("{}: {}", key, values.join(", ").red()),
        }
    }
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

fn get_unique_values(
                    keys: &Vec<&str>, 
                    get_values: bool, 
                    log: &Value,
                    value: &str,
                    mut uniques: &mut HashMap<String, u64>
                ) {
    // Get all field names across all logs
    if !check_key_value(&log, &keys, &value) { return; }
    // get all uniqued values of a given field
    traverse_json_values_unique(&log, &keys, &mut uniques);
}

fn get_value_type(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string()
    }
}

/*
   Recursively traverse Json structure to build dot delimited key paths
   and also report key value types
*/
fn traverse_json_key(json: &Value, prefix: &str, paths: &mut HashMap<String, HashSet<String>>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_prefix = get_new_prefix(&prefix, key);
                traverse_json_key(value, &new_prefix, paths);
            }
        }
        Value::Array(vec) => {
            if let Some(first_element) = vec.first() {
                traverse_json_key(first_element, prefix, paths);
            }
            let entry = paths.entry(prefix.to_owned()).or_insert(HashSet::new());
            entry.insert(get_value_type(&json));
            // get the type of the first value found in the array and append to the HashSet
            if !json.as_array().unwrap().is_empty() {
                let first_value = json.as_array().unwrap().first().unwrap();
                entry.insert(get_value_type(&first_value));
            } else {
                entry.insert("none".to_string());
            }
        }
        _ => {
            let entry = paths.entry(prefix.to_owned()).or_insert(HashSet::new());
            entry.insert(get_value_type(&json));
        }
    }
}

fn get_unique_keys(
                    keys: &Vec<&str>, 
                    get_values: bool, 
                    log: &Value,
                    value: &str
                ) -> HashMap<String, HashSet<String>> {
    let mut uniques = HashMap::new();
    // Get all field names across all logs
    if keys.is_empty() {
        traverse_json_key(log, &"".to_string(), &mut uniques);
    } else {
        if !check_key_value(&log, &keys, &value) { return uniques; }
        traverse_json_key(log, &"".to_string(), &mut uniques);
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

// Does the dot delimited Json key path exist?
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
        value,
        key_sort
    ) = get_args()?;

    let stdin = io::stdin();

    if !get_uniques { print_header(&fields, &delim) };
    
    let mut unique_values: HashMap<String, u64> = HashMap::new();
    let mut unique_keys: HashMap<String, HashSet<String>> = HashMap::new();
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
            if get_values {
               get_unique_values(&keys, get_values, &log, &value, &mut unique_values);
            } else {
                for (k, v) in get_unique_keys(&keys, get_values, &log, &value) {
                    let entry = unique_keys.entry(k).or_insert(HashSet::new());
                    entry.extend(v);
                }
            }
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
    if get_uniques {
        if get_values { 
            print_unique_values(&unique_values, key_sort);
        } else {
            print_unique_keys(&unique_keys);
        }
    }
    Ok(())
}

fn get_args() -> io::Result<(String, String, bool, String, bool, String, bool)> {
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
    let mut key_sort = false;
    for arg in args {
        match arg.as_str() {
            "-f" | "--fields" => get_fields = true,
            "-d" | "--delimiter" => get_delim = true,
            "-k" | "--key" => get_key = true,
            "-s" | "--string" => get_string = true,
            "-u" | "--unique" => get_uniques = true,
            "-v" | "--values" => get_values = true,
            "-z" | "--valuesort" => key_sort = true,
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
    } else if !string.is_empty() && key.is_empty() {
        println!("If '--string' is used then '--key' must be used.");
        print_help();
    }
    Ok((fields, delim, get_uniques, key, get_values, string.to_lowercase(), key_sort))
}


fn print_help() {
    let help = "
Author: Brian Kellogg
License: MIT
Purpose: Extract json fields and values in various ways.

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
        - print a uniqued list of all values found in the key 'key_name' across all logs
    cat logs.json | jve --unique --values --key 'key_name' -z
        - print a uniqued list of all values found in the key 'key_name' across all logs 
          and sort by the values, not the count of each unique value

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
                                      all logs and their data types
                                    - if the field is an array: array[data_type]
                                      empty array: array
                                    - if more than one data type is listed for a field
                                      then there are at least two logs with the same
                                      field name but containing differing value
                                      types
                                    - unique key names of logs wherein the given 
                                      key exists
                                    - if '--values' is also specified, list all the
                                      unique values of the specified key '--key'
                                    - Nested key names will be dot delimited
    -v, --values                    Must be used along with '--unique' and '--key'
                                    - print the unique values of the specified key
    -z, --valuesort                 Sort unique values by value instead of count

NOTE:   If a key is an array or the key name occurs in an array, 
        this program will concatenate all array key values into a 
        delimited quoted string across all array elements.
";
    println!("{}", help);
    process::exit(1)   
}