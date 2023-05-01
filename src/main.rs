extern crate serde_json;

use std::io;
use std::{env, process};
use serde_json::{Value};
use std::collections::HashSet;

fn print_results(output: &Vec<String>, split_fields: Vec<&str>, delim: &str) {
    if delim.eq("\\n") {
        split_fields.iter().zip(output.iter())
            .map(|(a, b)| format!("{}: {}", a, b))
            .for_each(|o| println!("[*] {}", o));
        println!();
        return
    }
    if delim.eq("\\t") {
        println!("{}", output.join("\t"));
        return
    }
    println!("{}", output.join(delim));
    return
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

fn get_field_value(json: &Value, name: &str)  -> io::Result<Value> {
    let value = match json.get(name) {
        Some(v) => v.to_owned(),
        None => Value::Null,
    };
    Ok(value)
}

fn get_array(input: &Value, name: &String) -> io::Result<Vec<Value>> {
    let js = match input[name].as_array() {
        Some(j) => j.to_owned(),
        None => Vec::new(),
    };
    Ok(js)
}

fn get_field(input: Value, name: &String) -> io::Result<(Value, bool, Value)> {
    let mut is_array = false;
    if let Some(_js) = input[name].as_array() {
        is_array = true;
    }
    let value = get_field_value(&input, name)?;
    Ok((value, is_array, input))
}

fn get_fields_array(array: &Vec<Value>, names: Vec<&str>, results: &mut Vec<Value>) -> io::Result<()> {
    let mut is_array = false;
    for entry in array {
        let fields = names.clone();
        let mut track_names = names.clone();
        let mut value = Value::Null;
        for n in fields {
            track_names.remove(0);
            (value, is_array, _) = get_field(entry.clone(), &n.to_string())?;
            if is_array && track_names.len() != 0 { // if there are no field names left assume we want to extract the last field
                println!("{:?}", value);
                println!("{:?}", n);
                let js = match value.as_array() {
                    Some(j) => j.to_owned(),
                    None => continue,
                };
                get_fields_array(&js, track_names.clone(), results)?;
            }
        }
        if !value.is_null() { results.push(value); }
    }
    Ok(())
}

fn join_values(array: &Vec<Value>, delim: &String) -> io::Result<String> {
    let temp: Vec<String> = array.into_iter().map(|n| n.to_string()).collect();
    Ok(format!("\"{}\"", temp.join(delim)))
}

fn get_fields(log: &Value, fields: &String, delim: &String) -> io::Result<()> {
    let split_fields: Vec<&str> = fields.split(",").collect();
    let mut output = Vec::new(); // vec to build final output
    let mut results = Vec::new();
    for field in split_fields.iter() {
        let names: Vec<&str> = field.split(".").collect();
        let mut track_names = names.clone(); // needed for when we hit a field that is an array
        let mut is_array = false; // if we hit a value that is an array, we need to treat it differently
        let mut json = log.clone();
        let mut value = Value::Null;
        let mut previous_value = Value::Null; // track previous json object
        let mut previous_name = String::new(); // track previously used json field name
        let mut array_values_concat = String::new();
        for n in names {
            if is_array { // detecting arrays is working, but logic is horrible I think, prob can be done better
                let js = get_array(&previous_value.clone(), &previous_name)?;
                get_fields_array(&js, track_names.clone(), &mut results)?;
                array_values_concat = join_values(&results, delim)?;
                break; // if we hit an array we treat the rest of the parsing differently
            }
            track_names.remove(0); // keep track of field names already used by removing them from this vec
            (value, is_array, previous_value) = get_field(json, &n.to_string())?;
            previous_name = n.to_string(); // if we run into an array, we need the previously used field name to start parsing the array
            json = value.clone();
        }
        if is_array && value.is_null() { value = previous_value.clone() }
        if is_array && !array_values_concat.is_empty() {
            output.push(array_values_concat);
            continue;
        }
        output.push(value.to_string());
    }
    print_results(&output, split_fields, delim);
    Ok(())
}

fn print_vec(values: &Vec<String>) {
    for v in values {
        println!("{}", v);
    }
}

fn print_uniques(mut uniques: Vec<String>) {
    uniques.sort();
    uniques.dedup();
    uniques.retain(|v| v != "");
    print_vec(&uniques);
}

fn get_key_value(json: &Value, key: &str) -> (bool, String) {
    let keys: Vec<&str> = key.split('.').collect();
    let mut current_json = json;
    for k in keys {
        match current_json.get(k) {
            Some(val) => current_json = val,
            None => return (false, String::new())
        }
    }
    let value = current_json.to_string().to_lowercase();
    (true, value)
}

fn print_header(get_uniques: bool, fields: &str, delim: &str) {
    if !get_uniques{
        if !delim.eq("\\n") { println!("{}", fields.replace(",", &delim)) }
    }
}

fn get_json_values(json: &Value, path: &str) -> Vec<String> {
    let mut result = Vec::new();
    let keys: Vec<&str> = path.split('.').collect();
    get_values_recursive(json, &keys, &mut result);
    result
}

fn get_values_recursive(json: &Value, keys: &[&str], result: &mut Vec<String>) {
    if let Some(key) = keys.first() {
        if let Some(value) = json.get(*key) {
            if value.is_array() {
                for item in value.as_array().unwrap() {
                    get_values_recursive(item, &keys[1..], result);
                }
            } else if value.is_object() {
                get_values_recursive(value, &keys[1..], result);
            } else if keys.len() == 1 {
                if let Some(value_str) = value.as_str() {
                    result.push(value_str.to_string());
                }
            }
        }
    }
}

fn traverse_json(json: &Value, prefix: String, paths: &mut HashSet<String>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map {
                let new_prefix = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                traverse_json(value, new_prefix, paths);
            }
        }
        Value::Array(vec) => {
            if let Some(first_element) = vec.first() {
                traverse_json(first_element, prefix.clone(), paths);
            }
        }
        _ => {
            paths.insert(prefix);
        }
    }
}

fn get_field_paths(json: &Value) -> Vec<String> {
    let mut paths = HashSet::new();
    traverse_json(json, "".to_string(), &mut paths);
    let mut paths_vec: Vec<String> = paths.into_iter().collect();
    paths_vec.sort();
    paths_vec
}

fn process_uniques(
                    field_name: &str, 
                    get_values: bool, 
                    log: &Value,
                    value: &str
                ) -> io::Result<Vec<String>> {
    let mut uniques: Vec<String> = Vec::new();
    // Get all field names across all logs
    if field_name.is_empty() {
        let mut path: Vec<String> = Vec::new();
        uniques.extend(get_field_paths(&log));
    } else {
        let (key_exists, key_value) = get_key_value(&log, &field_name);
        // return if the string specified is not found in the key's value
        if !value.is_empty() && !key_value.contains(&value) { return Ok(uniques) }
        // get all uniqued values of a given field
        if get_values {
            uniques.extend(get_json_values(&log, field_name));
        // get all uniqued field names where a given field exists in the log
        } else if key_exists {
            let mut path: Vec<String> = Vec::new();
            uniques.extend(get_field_paths(&log));
        }
    }
    Ok(uniques)
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

    print_header(get_uniques, &fields, &delim);
    
    let mut uniques: Vec<String> = Vec::new();

    for line in stdin.lines() {
        let l = match line {
            Ok(o) => o,
            Err(_) => continue,
        };

        let log = string_to_json(l)?;

        // We are only looking for unique field names or values in a given field
        if get_uniques {
            uniques.extend(process_uniques(&field_name, get_values, &log, &value)?);
            continue;
        }

        // only print out logs where the given key exists  
        // and/or its value contains specified value
        if !field_name.is_empty() {
            let (key_exists, key_value) = get_key_value(&log, &field_name);
            if !key_exists { continue; }
            if !value.is_empty() && !key_value.contains(&value) { continue; }
            get_fields(&log, &fields, &delim)?;
            continue;
        }

        // we just want all fields specified 
        // including null results from log not having those fields
        get_fields(&log, &fields, &delim)?;
    }

    // if we were only looking for uniques, print what was found
    if get_uniques { print_uniques(uniques) }
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
    let mut get_value = false;
    let mut value = String::new();
    for arg in args {
        match arg.as_str() {
            "-f" | "--fields" => get_fields = true,
            "-d" | "--delimiter" => get_delim = true,
            "-k" | "--key" => get_key = true,
            "-s" | "--string" => get_value = true,
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
                } else if get_value {
                    value = arg.to_string();
                    get_value = false;
                }
            }
        }
    }
    Ok((fields, delim, get_uniques, key, get_values, value.to_lowercase()))
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