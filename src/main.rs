extern crate serde_json;

use std::io::{self, Read, BufRead};
use serde_json::{Value, json};
use std::{mem, env};


fn print_results(output: &Vec<String>, delim: &String) {
    let mut results = String::new();
    for o in output {
        if delim.eq("\\n") {
            if !o.is_empty() { println!("{}", o); }
            continue;
        }
        results.push_str(o);
        if delim.eq("\\t") {
            results.push('\t');
            continue;
        }
        results.push_str(delim);
    }
    if !results.is_empty() { 
        for _r in 0..delim.len() { results.pop(); }
        println!("{}", results);
    }
}

fn string_to_json(input: String) -> io::Result<Value> {
    let json: Value = {
        let this = serde_json::from_str(&input);
        match this {
            Ok(t) => t,
            Err(e) => {
                return Ok(Value::Null);
            },
        }
    };
    Ok(json)
}

fn get_array_values(input: Value, name: &String)  -> io::Result<String> {
    let mut value = String::new();
    if let Some(js) = input.as_array() {
        for j in js {
            let temp = match j.get(name) {
                Some(v) => v.to_string(),
                None => "".to_string(),
            };
            value.push_str(&temp);
            value.push_str(",");
        }
        value.pop();
    }
    Ok(value)
}

fn get_field_value(json: Value, name: &String)  -> io::Result<Value>{
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
    let mut value =Value::Null;
    if let Some(js) = input[name].as_array() {
        return Ok((value, true, input.to_owned()))
    } else {
        value = get_field_value(input.clone(), name)?;
    }
    Ok((value, false, input))
}

fn get_fields_array(array: &Vec<Value>, names: Vec<&str>) -> io::Result<Vec<Value>> {
    let mut previous = Value::Null;
    let mut is_array = false;
    let mut js = Vec::new();
    let mut output: Vec<Value> = Vec::new();
    for entry in array {
        let fields = names.clone();
        let mut track_names = names.clone();
        let mut results: Vec<Value> = Vec::new();
        let mut value = Value::Null;
        for n in fields {
            track_names.remove(0);
            if is_array {
                js = get_array(&value.clone(), &n.to_string())?;
                results = get_fields_array(&js, track_names.clone())?;
            }
            (value, is_array, previous) = get_field(entry.clone(), &n.to_string())?;
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
    let mut results: Vec<String> = Vec::new();
    for v in array {
        results.push(v.to_string());
    }
    let results_concat = results.join(delim);
    Ok(format!("\"{}\"", results_concat))
}

fn get_fields(input: String, fields: String, delim: &String) -> io::Result<()> {
    let split_fields = fields.split(",");
    let orig_json = string_to_json(input)?;
    let mut output = Vec::new();
    for field in split_fields {
        let mut names: Vec<&str> = field.split(".").collect();
        let mut track_names = names.clone(); // needed for when we hit a field that is an array
        let mut is_array = false; // if we hit a value that is an array, we need to treat it differently
        let mut json = orig_json.clone();
        let mut value = Value::Null;
        let mut previous_value = Value::Null;
        let mut previous_name = String::new();
        let mut js = Vec::new();
        let mut array_results: Vec<Value> = Vec::new();
        let mut array_values_concat = String::new();
        for n in names {
            if is_array {
                js = get_array(&previous_value.clone(), &previous_name)?;
                array_results = get_fields_array(&js, track_names.clone())?;
                array_values_concat = join_values(&array_results, delim)?;
                break;
            }
            track_names.remove(0);
            (value, is_array, previous_value) = get_field(json, &n.to_string())?;
            previous_name = n.to_string();
            json = value.clone();
        }
        if is_array && value.is_null() { value = previous_value.clone() }
        if is_array && !array_values_concat.is_empty() {
            output.push(array_values_concat);
            continue;
        }
        output.push(value.to_string());
    }
    print_results(&output, delim);
    Ok(())
}

fn get_args() -> io::Result<(String, String)> {
    let args: Vec<String> = env::args().collect();
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
    let mut stdin = stdin.lock(); // locking is optional
    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_line(&mut line) {
        if n_bytes == 0 { break }
        get_fields(line.to_owned(), fields.to_string(), delim)?;
        line.clear();
    }
    Ok(())
}