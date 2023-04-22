extern crate serde_json;

use std::io;
use std::{env, process};
use serde_json::{Value};

fn print_results(output: &Vec<String>, split_fields: Vec<&str>, delim: &String) {
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
    let mut js = Vec::new();
    js = match input[name].as_array() {
        Some(j) => j.to_owned(),
        None => js,
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

fn get_fields_array(array: &Vec<Value>, names: Vec<&str>) -> io::Result<Vec<Value>> {
    let mut output: Vec<Value> = Vec::new();
    for entry in array {
        let mut is_array = false;
        let fields = names.clone();
        let mut track_names = names.clone();
        let mut results: Vec<Value> = Vec::new();
        let mut value = Value::Null;
        for n in fields {
            track_names.remove(0);
            if is_array && track_names.len() != 0 { // if there are no field names left assume we want to extract the last field
                let js = get_array(&value.clone(), &n.to_string())?;
                results = get_fields_array(&js, track_names.clone())?;
            }
            (value, is_array, _) = get_field(entry.clone(), &n.to_string())?;
        }
        if !results.is_empty() {
            for r in results {
                output.push(r);
            }
            continue;
        }
        output.push(value);
    }
    Ok(output)
}

fn join_values(array: &Vec<Value>, delim: &String) -> io::Result<String> {
    let temp: Vec<String> = array.into_iter().map(|n| n.to_string()).collect();
    Ok(format!("\"{}\"", temp.join(delim)))
}

fn get_fields(fields: &String, delim: &String, log: &Value) -> io::Result<()> {
    let split_fields: Vec<&str> = fields.split(",").collect();
    let mut output = Vec::new(); // vec to build final output
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
                let array_results = get_fields_array(&js, track_names.clone())?;
                array_values_concat = join_values(&array_results, delim)?;
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

/*
    path: Track the dot delimited field path as we traverse the JSON structure
    result: Mutable array to hold additions through recursive calls of the function
*/
fn traverse_json(json: &Value, path: &mut Vec<String>, result: &mut Vec<String>) {
    match json {
        Value::Object(map) => {
            for (key, value) in map.iter() {
                path.push(key.to_string());
                traverse_json(value, path, result);
                path.pop();
            }
        },
        _ => {
            let field_path = path.join(".");
            result.push(field_path);
        }
    }
}

fn get_value(log: &Value, field_path: &str) -> io::Result<String> {
    let fields = field_path.split('.').collect::<Vec<&str>>();
    let mut value = log;

    for field in fields {
        value = match value.get(field) {
            Some(v) => &v,
            _ => &Value::Null,
        };
    }
    Ok(value.to_string())
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
        traverse_json(&log, &mut path, &mut uniques);
    } else {
        let (key_exists, key_value) = get_key_value(&log, &field_name);
        // return if the string specified is not found in the key's value
        if !value.is_empty() && !key_value.contains(&value) { return Ok(uniques) }
        // get all uniqued values of a given field
        if get_values {
            uniques.push(get_value(&log, &field_name)?);
        // get all uniqued field names where a given field exists in the log
        } else if key_exists {
            let mut path: Vec<String> = Vec::new();
            traverse_json(&log, &mut path, &mut uniques);
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

        // only print out logs where the given field exists  
        // and/or its value contains specified value in the key's value
        if !field_name.is_empty() {
            let (key_exists, key_value) = get_key_value(&log, &field_name);
            if !key_exists { continue; }
            if !value.is_empty() && !key_value.contains(&value) { continue; }
            get_fields(&fields, &delim, &log)?;
            continue;
        }

        // we just want all fields specified 
        // including null results from log not having those fields
        get_fields(&fields, &delim, &log)?;
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
    cat logs.json | jve --unique --name 'key_name'
        - Collect and print a uniqued list of all key names found in logs with 
          the specified 'key_name'
    cat logs.json | jve --unique --values --key 'key_name'
        - print a uniqued list of all values found in the key 'key_name' 
          across all logs

Options:
    -d, --delimiter ','             Value to use to seperate field value output
    -f, --fields 'a.b.c.d,a.b.e'    Comma seperated list of fields in dot notation
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