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

fn get_array_values(input: &Value, name: &String)  -> io::Result<String> {
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

fn get_field_value(json: &Value, name: &String)  -> io::Result<Value>{
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

fn get_field(input: &Value, name: &String) -> io::Result<(Value, bool, Value)> {
    let mut value =Value::Null;
    if let Some(js) = input[name].as_array() {
        return Ok((value, true, input.to_owned()))
    } else {
        value = get_field_value(input, name)?;
    }
    Ok((value, false, input.to_owned()))
}

fn walk_path(array: &Vec<Value>, names: Vec<&str>) {

}

fn get_fields(input: String, fields: String, delim: &String) -> io::Result<()> {
    let split_fields = fields.split(",");
    let orig_json = string_to_json(input)?;
    let mut output = Vec::new();
    for field in split_fields {
        let mut names: Vec<&str> = field.split(".").collect();
        let mut json = orig_json.clone();
        let mut value = Value::Null;
        let mut is_array = false;
        let mut previous = Value::Null;
        let mut js = Vec::new();
        for n in names {
            if is_array {
                js = get_array(&value, &n.to_string())?;
                walk_path(&js, names..clone());
                if js.is_empty() {
                    (value, is_array, previous) = get_field(&json, &n.to_string())?;
                    continue;
                }
                value = serde_json::Value::String(get_array_values(&value, &n.to_string())?);
            }
            (value, is_array, previous) = get_field(&json, &n.to_string())?;
            json = value.clone();
        }
        if is_array && value.is_null() { value = previous.clone() }
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