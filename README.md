# Json Value Extractor
Cmd line utility that accepts json via standard in (piping) and extracts values from json fields.

```
Author: Brian Kellogg
License: MIT
Purpose: Extract json field values

Usage: 
    fmd [--pretty | -p] ([--strings|-s] #) <file path> ([--depth | -d] #)
    fmd --pretty --depth 3 --extensions \"exe,dll,pif,ps1,bat,com\"
    fmd --pretty --depth 3 --extensions \"not:exe,dll,pif,ps1,bat,com\"
        This will process all files that do not have the specified extensions.

Options:
    -d, --delimieter \",\"          Value to use to seperate field value output
    -f, --fields \"a.b.c.d,a.b.e\"  Comma seperated list of fields in dot notation

NOTE:   If a field is an array or the field name occurs in an array, 
        this program will concatinate all array field values into a comma 
        seperated quoted string across all arrays elements.
```