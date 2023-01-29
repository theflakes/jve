extern crate serde_json;

use std::io;
use std::{env, process};
use serde_json::Value;


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

fn get_field_value(json: Value, name: &String)  -> io::Result<Value> {
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
    let value = get_field_value(input.clone(), name)?;
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

fn get_fields(input: String, fields: String, delim: &String) -> io::Result<()> {
    let split_fields: Vec<&str> = fields.split(",").collect();
    let orig_json = string_to_json(input)?; // track original json object so we can start at the beginning of it for each field
    let mut output = Vec::new(); // vec to build final output
    for field in split_fields.iter() {
        let names: Vec<&str> = field.split(".").collect();
        let mut track_names = names.clone(); // needed for when we hit a field that is an array
        let mut is_array = false; // if we hit a value that is an array, we need to treat it differently
        let mut json = orig_json.clone();
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

fn get_args() -> io::Result<(String, String)> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 { print_help(); }
    let mut fields = String::new();
    let mut delim = String::new();
    let mut get_fields = false;
    let mut get_delim = false;
    for arg in args {
        match arg.as_str() {
            "-f" | "--fields" => get_fields = true,
            "-d" | "--delimiter" => get_delim = true,
            _ => {
                if get_fields {
                    fields = arg.to_string();
                    get_fields = false;
                } else if get_delim {
                    delim = arg.to_string();
                    get_delim = false;
                }
            }
        }
    }
    Ok((fields, delim))
}

fn main() -> io::Result<()> {
    let (fields, delim) = &get_args()?;

    let stdin = io::stdin();

    if !delim.eq("\\n") {println!("{}", &fields.replace(",", &delim))}

    for line in stdin.lines() {
        let l = match line {
            Ok(o) => o,
            Err(_) => continue,
        };
        get_fields(l, fields.to_string(), delim)?;
    }
    Ok(())
}

fn print_help() {
    let help = "
Author: Brian Kellogg
License: MIT
Purpose: Extract json field values

JVE - Json Value Extractor

This program accepts piping line delimited json input via output from some previous command.

Usage: 
    cat logs.json | jve --delimiter \",\" --fields \"filename,hashes.md5,hashes.ssdeep\"
        - comma seperated output
    cat logs.json | jve -d \"\\n\" -f \"filename,hashes.md5,hashes.ssdeep\"
        - output to a new line for each field
    cat logs.json | jve -d \"\\t\" -f \"filename,hashes.md5,hashes.ssdeep\"
        - tab seperated output

Options:
    -d, --delimiter \",\"           Value to use to seperate field value output
    -f, --fields \"a.b.c.d,a.b.e\"  Comma seperated list of fields in dot notation

NOTE:   If a field is an array or the field name occurs in an array, 
        this program will concatinate all array field values into a delimited
        quoted string across all array elements.
";
    println!("{}", help);
    process::exit(1)   
}