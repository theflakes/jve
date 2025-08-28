extern crate serde_json;
extern crate itertools;

use std::io;
use std::{env, process};
use serde_json::Value;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;
use colored::Colorize;


fn print_results(
    output: &[String], 
    split_fields: Vec<&str>, 
    delim: &str
) 
{
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
    if !results.is_empty() && !results.is_empty() 
        && !results.eq("\"\"") && !results.eq("[\"\"]")
            { println!("{}", results); }
}




fn get_first_elem(
    set: &HashSet<String>
) -> Option<&String> 
{
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
    format!("\"{}\"", array.iter().join(","))
}


// Setup for havesting targeted key's values
fn get_key_values(
    json: &Value, 
    field_paths: &str, 
    delim: &str
) 
{
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
fn traverse_json_value(
    json: &Value, 
    field_names: &[&str], 
    values: &mut HashSet<String>
) 
{
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
fn traverse_json_values_unique(
    json: &Value, 
    field_names: &[&str], 
    uniques: 
    &mut HashMap<String, u64>
) 
{
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


fn print_unique_values(
    mut uniques: &HashMap<String, u64>, key_sort: bool
) 
{
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
    for (k, v) in u { println!("{}:{}", k, v) }
}


fn format_values(
    values_map: HashMap<String, usize>
) -> String 
{ 
    let mut values: Vec<(String, usize)> = values_map.into_iter().collect(); 
    values.sort_by(|a, b| a.0.cmp(&b.0));
    let formatted_values: Vec<String> = values.into_iter() 
        .map(|(k, v)| format!("{},{}", k, v))
        .collect(); 
    match formatted_values.len() { 
        1 => formatted_values.join(",").green().to_string(),
        _ => formatted_values.join(",").red().to_string(),
    }
}


fn print_unique_keys(
    uniques: &HashMap<String, (HashMap<String, usize>, usize)>
) 
{ 
    for key in uniques.keys().sorted() { 
        let v = uniques[key].clone(); 
        let values = format_values(v.0); 
        let count = v.1.to_string().yellow().to_string();
        println!("{},{},{}", key, count, values); 
    } 
}


// If not using new line delim, print field header
fn print_header(
    fields: &str, 
    delim: &str
) 
{
    if delim.eq("\\n") { return; }
    match delim {
        "\\t" => println!("{}", fields.replace(",", "\t")),
        _ => println!("{}", fields.replace(",", delim))
    }
}


// Build dot delimited key paths as we traverse the Json structure
fn get_new_prefix(
    prefix: &str, 
    key: &str
) -> String
{
    let new_prefix = if prefix.is_empty() {
        key.to_string()
    } else {
        format!("{}.{}", prefix, key)
    };
    new_prefix
}


fn get_unique_values(
    keys: &Vec<&str>, 
    get_values: bool, 
    log: &Value,
    value: &str,
    mut uniques: &mut HashMap<String, u64>
) 
{
    // Get all field names across all logs
    if !check_key_value(log, keys, value) { return; }
    // get all uniqued values of a given field
    traverse_json_values_unique(log, keys, uniques);
}


fn get_value_type(
    value: &Value
) -> String 
{
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string()
    }
}


fn update_map_key_type_count(
    map: &mut HashMap<String, usize>, 
    key_type: &str
) 
{
    if let Some(count) = map.get_mut(key_type) { 
            *count += 1; 
    } else {
        map.insert(key_type.to_owned(), 1);
    }
}


fn update_or_insert_key_type(
    json_value: &Value, 
    map: &mut HashMap<String, usize>
) 
{
    let mut key_type = String::new();
    if json_value.is_array() {
        if !json_value.as_array().unwrap().is_empty() {
            let value_type = json_value.as_array().unwrap().first().unwrap();
            key_type = format!("array[{}]", get_value_type(value_type));
        } else {
            key_type = "array[none]".to_string();
        }
    } else {
        key_type = get_value_type(json_value);
    }
    update_map_key_type_count(map, &key_type);        
}


fn update_key_info(
    json_value: &Value, 
    prefix: &str, 
    paths: &mut HashMap<String, (HashMap<String, usize>, usize)>
) 
{
    let mut entry = paths.entry(prefix.to_owned())
        .or_insert((HashMap::new(), 0));
    update_or_insert_key_type(json_value, &mut entry.0);
    entry.1 += 1;
}


/*
   Recursively traverse Json structure to build dot delimited key paths
   and also report key value types
*/
fn traverse_json_key(
    json_value: &Value, 
    prefix: &str, 
    paths: &mut HashMap<String, (HashMap<String, usize>, usize)>
) 
{
    match json_value {
        Value::Object(map) => {
            if map.is_empty() && !prefix.is_empty() {
                update_key_info(json_value, prefix, paths);
            }
            for (key, value) in map {
                let new_prefix = get_new_prefix(prefix, key);
                traverse_json_key(value, &new_prefix, paths);
            }
            return;
        }
        Value::Array(vec) => {
            if !prefix.is_empty() {
                update_key_info(json_value, prefix, paths);
            }
            for element in vec {
                if element.is_object() || element.is_array() {
                    traverse_json_key(element, prefix, paths);
                }
            }
            return;
        }
        _ => {}
    }
    if !prefix.is_empty() {
        update_key_info(json_value, prefix, paths);
    }
}


fn get_unique_keys(
    keys: &Vec<&str>, 
    get_values: bool, 
    log: &Value,
    value: &str,
    paths: &mut HashMap<String, (HashMap<String, usize>, usize)>
) 
{
    // Get all field names across all logs
        if keys.is_empty() || check_key_value(log, keys, value) { 
        traverse_json_key(log, "", paths); 
    }
}


fn found_in_vec(
    values: &HashSet<String>, 
    value: &str
) -> bool 
{
    for u in values {
        if u.to_lowercase().contains(value) {
            return true
        }
    }
    false
}


// Does the dot delimited Json key path exist?
fn path_exists(
    json: &Value, 
    keys: &[&str]
) -> bool 
{
    if let Some((first_key, remaining_keys)) = keys.split_first() {
        match json {
            Value::Object(map) => {
                if let Some(value) = map.get(*first_key) {
                    if remaining_keys.is_empty() {
                        true
                    } else {
                        path_exists(value, remaining_keys)
                    }
                } else {
                    false
                }
            }
            Value::Array(array) => {
                if let Some(first_element) = array.first() {
                    path_exists(first_element, keys)
                } else {
                    false
                }
            }
            _ => false,
        }
    } else {
        false
    }
}


// Verify Key Value pair exist
fn check_key_value(
    log: &Value, 
    keys: &Vec<&str>, 
    value: &str
) -> bool 
{
    if value.is_empty() { return path_exists(log, keys) }
    let mut values: HashSet<String> = HashSet::new();
    traverse_json_value(log, keys, &mut values);
    if values.is_empty() { return false; }
    found_in_vec(&values, value)
}

fn main() -> io::Result<()> {
        let args = get_args()?;

        let stdin = io::stdin();
    let deserializer = serde_json::Deserializer::from_reader(stdin);
    let iterator = deserializer.into_iter::<Value>();

    if args.all_values {
        let mut paths: HashMap<String, (HashMap<String, usize>, usize)> = HashMap::new();
        let all_logs: Vec<Value> = iterator.filter_map(Result::ok).collect();
        for log in &all_logs {
            get_unique_keys(&Vec::new(), false, log, "", &mut paths);
        }

        for key in paths.keys().sorted() {
            println!("--- {} ---", key);
            let mut unique_values: HashMap<String, u64> = HashMap::new();
            let key_vec: Vec<&str> = key.split('.').collect();
            for log in &all_logs {
                traverse_json_values_unique(log, &key_vec, &mut unique_values);
            }
            print_unique_values(&unique_values, args.key_sort);
        }
        return Ok(());
    }

        if !args.get_uniques { print_header(&args.fields, &args.delim) };
    
        let mut unique_values: HashMap<String, u64> = HashMap::new();
    let mut paths: HashMap<String, (HashMap<String, usize>, usize)> = HashMap::new();
    let no_whitespace = args.key.replace(char::is_whitespace, "");
    let mut keys: Vec<&str> = no_whitespace.split('.').collect();
    keys.retain(|&k| !k.is_empty());

    for item in iterator {
        let log = match item {
            Ok(o) => o,
            Err(_) => {continue},
        };

                // We are only looking for unique field names or values in a given field
        if args.get_uniques {
                        if args.get_values {
               get_unique_values(&keys, args.get_values, &log, &args.string, &mut unique_values);
            } else {
                get_unique_keys(&keys, args.get_values, &log, &args.string, &mut paths);
            }
            continue;
        }

        // only print out logs where the given key exists  
        // and/or its value contains specified value
        if !keys.is_empty() {
                        if !check_key_value(&log, &keys, &args.string) { continue; }
                        get_key_values(&log, &args.fields, &args.delim);
            continue;
        }

        // we just want all fields specified 
        // including null results from log not having those fields
                    get_key_values(&log, &args.fields, &args.delim);
    }

        // if we were only looking for uniques, print what was found
    if args.get_uniques {
                if args.get_values { 
            print_unique_values(&unique_values, args.key_sort);
        } else {
            println!("Key,Count,Type,Count");
            print_unique_keys(&paths);
        }
    }
    Ok(())
}


struct CliArgs {
    fields: String,
    delim: String,
    get_uniques: bool,
    key: String,
    get_values: bool,
    string: String,
    key_sort: bool,
    all_values: bool,
}

fn get_args() -> io::Result<CliArgs> {
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
    let mut all_values = false;
    let mut iter = args.iter().skip(1);
    for arg in iter {
        if arg.starts_with("--") {
            match arg.as_str() {
                "--all-values" => all_values = true,
                "--delimiter" => get_delim = true,
                "--fields" => get_fields = true,
                "--key" => get_key = true,
                "--string" => get_string = true,
                "--unique" => get_uniques = true,
                "--values" => get_values = true,
                "--valuesort" => key_sort = true,
                _ => {}
            }
        } else if arg.starts_with("-") {
            for c in arg.chars().skip(1) {
                match c {
                    'a' => all_values = true,
                    'd' => get_delim = true,
                    'f' => get_fields = true,
                    'k' => get_key = true,
                    's' => get_string = true,
                    'u' => get_uniques = true,
                    'v' => get_values = true,
                    'z' => key_sort = true,
                    _ => {}
                }
            }
        } else if get_fields {
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
    if fields.is_empty() ^ delim.is_empty() {
        println!("If either '--delimiter' or '--fields' is used, both must be used.");
        print_help();
    } else if !string.is_empty() && key.is_empty() {
        println!("If '--string' is used then '--key' must be used.");
        print_help();
    }
        Ok(CliArgs {
        fields,
        delim,
        get_uniques,
        key,
        get_values,
        string: string.to_lowercase(),
        key_sort,
        all_values,
    })
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
        - comma separated output
    cat logs.json | jve -d '\\n' -f 'filename,hashes.md5,hashes.ssdeep'
        - output to a new line for each field
    cat logs.json | jve -d '\\t' -f 'filename,hashes.md5,hashes.ssdeep'
        - tab separated output
    cat logs.json | jve -d ',' -f 'filename,hashes.md5' --key 'path'
        - comma separated list of all fields only where the key named 'path' exists
    cat logs.json | jve -d ',' -f 'filename,hashes.md5' -k 'path' --string '/home/evil'
        - comma separated list of all fields only where the key named 'path' exists
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
    cat logs.json | jve --all-values
        - print a uniqued list of all values for every key across all logs

Options:
    -a, --all-values                Get all unique values for all keys
    -d, --delimiter ','             Value to use to separate  key value output
                                    - when using a new line delimiter, array values
                                      will be comma separated
    -f, --fields 'a.b.c.d,a.b.e'    Comma separated list of keys in dot notation
    -k, --key 'name_of_key'         Only examine logs where the specified key exists
    -s, --string 'string'           Only examine logs where the specified key's value
                                    contains the specified string
                                    - must be used with '--key'
                                    - case insensitive match
    -u, --unique                    Get uniqued entries for: 
                                    - if used by itself, all field names across 
                                      all logs, count of occurrences across all logs, 
                                      and their data types, data types will also 
                                      include a count of occurrences across all logs
                                    - if the field is an array: array[data_type]
                                      empty array: array[none]
                                    - if more than one data type is listed for a field
                                      then there are at least two logs with the same
                                      field name but containing differing value
                                      types
                                    - unique key names of logs wherein the given 
                                      key exists when '--key <key dot delimited name>' 
                                      is specified
                                    - if '--values' is also specified, list all the
                                      unique values of the specified key 
                                      '--key <key dot delimited name>'
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