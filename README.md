# Json Value Extractor
Cmd line utility that accepts json via standard in (piping) and extracts values from json fields.

```
Author: Brian Kellogg
License: MIT
Purpose: Extract json field values

JVE - Json Value Extractor

This program accepts piping line delimited json input via output from some previous command.

Usage: 
    cat logs.json | jve --delimiter "," --fields "filename,hashes.md5,hashes.ssdeep"
        - comma seperated output
    cat logs.json | jve -d "\n" -f "filename,hashes.md5,hashes.ssdeep"
        - output to a new line for each field
    cat logs.json | jve -d "\t" -f "filename,hashes.md5,hashes.ssdeep"
        - tab seperated output

Options:
    -d, --delimieter ","          Value to use to seperate field value output
    -f, --fields "a.b.c.d,a.b.e"  Comma seperated list of fields in dot notation

NOTE:   If a field is an array or the field name occurs in an array, 
        this program will concatinate all array field values into a comma 
        seperated quoted string across all arrays elements.
```