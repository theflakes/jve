# Json Value Extractor
Cmd line Linux and Windows utility that accepts json via standard in (piping) and extracts values from json fields.

```
Author: Brian Kellogg
License: MIT
Purpose: Extract json field values

JVE - Json Value Extractor

This program accepts piping line delimited json input via output from some previous command.

Usage: 
    cat logs.json | jve --delimiter ',' --fields 'filename,hashes.md5,hashes.ssdeep'
        - comma seperated output
    cat logs.json | jve -d '\n' -f 'filename,hashes.md5,hashes.ssdeep'
        - output to a new line for each field
    cat logs.json | jve -d '\t' -f 'filename,hashes.md5,hashes.ssdeep'
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
                                      all logs and their data types
                                    - if the field is an array, the second data type
                                      will be that of the values, unless the array 
                                      is empty then there will not be a second data 
                                      type listed
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
```
### To Compile on Linux for static linking
```
sudo apt install musl-tools
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl --release
```
### Example output
```
fmd.exe .\fmd.exe | jve -d "," -f "filename,hashes.md5,entropy,binary.sections.sections.name,binary.sections.sections.entropy,binary.imports.imports.lib,binary.imports.imports.count"

filename,hashes.md5,entropy,binary.sections.sections.name,binary.sections.sections.entropy,binary.imports.imports.lib,binary.imports.imports.names
"fmd.exe","729e4a560c865f7cc28725337abcb4a0",6.3832226,"".text",".rdata",".data",".pdata","_RDATA",".reloc"","6.2971563,5.5931087,2.0857084,5.816629,3.3070078,5.4327927",""KERNEL32.dll","ADVAPI32.dll","bcrypt.dll"","101,2,3"
```
#### Example output parsing unique values from a common field across all JSON logs
```
cat .\files.json | .\jve --unique --values --key "hashes.md5"
"ee4e79fc4ade3bd7138537f7cd21b430"
"ee85b6c6c6b11b21ca5366b38bedf58a"
"f056cfc66651b933386b72bd11197b38"
"f17aaa27f0a9f1945833f8ce30f84248"
"f22095aaef76e2e5b2f870c2e0da7f54"
"f2b9a18f143c0213c55d74d01b0e01c6"
"f2fe47e47b9faa0646c2626a1b933f14"
"f31f030a35e983f31ceaf7cb4188c9e2"
"f3679a7fef38af94522fec6ad112adb2"
"f41b90b2f86bbdef77a8f643021cb442"
"f44a58405dbc1000ff607d0feaa61c70"
"f546bc07cc171fd67e4219cd7c840b81"
"f624c1e424863ea88d07faa1021cf80b"
"f62c0bb8d88f0f24140a4fa811cac4d8"
"f7f13171fe6177ba2ae285e125e52fe8"
"f829f05c9286170c2598589a45d0990d"
"f8ed4945674fd1dd8eb4de0394eaed26"
"f9a10da1d4a97c1c9772c2adad4b6e23"
"f9f108d33da837ae37336b9b528e1fbe"
"fa4a3dcce2fcf5328bead6fde5f52c95"
"fa5a792fdf9398cd686692d7c1b224f2"
"fe4219cdc6b63686e32a7c3cb16f0835"
"ff04321347f63e773591869e23d13b7f"
```
#### Example output using new line as a delimiter recursing through sub directories
```
fmd.exe c:\ -d 2 | jve -d "\n" --fields "filename,hashes.md5,entropy,binary.sections.sections.name,binary.sections.sections.entropy,ads.name,ads.bytes,ads.first_256_bytes,binary.imports.imports.lib,binary.imports.imports.names"

[*] filename: "$WINRE_BACKUP_PARTITION.MARKER"
[*] hashes.md5: "d41d8cd98f00b204e9800998ecf8427e"
[*] entropy: 0.0
[*] binary.sections.sections.name: ""
[*] binary.sections.sections.entropy: ""
[*] ads.name: """"
[*] ads.bytes: "0"
[*] ads.first_256_bytes: """"
[*] binary.imports.imports.lib: ""
[*] binary.imports.imports.names: ""

[*] filename: "desktop.ini"
[*] hashes.md5: "6383522c180badc4e1d5c30a5c4f4913"
[*] entropy: 3.5208218
[*] binary.sections.sections.name: ""
[*] binary.sections.sections.entropy: ""
[*] ads.name: """"
[*] ads.bytes: "174"
[*] ads.first_256_bytes: ""??????\r.\n.[...S.h.e.l.l.C.l.a.s.s.I.n.f.o.].\r.\n.L.o.c.a.l.i.z.e.d.R.e.s.o.u.r.c.e.N.a.m.e.=.@.%.S.y.s.t.e.m.R.o.o.t.%.\\.s.y.s.t.e.m.3.2.\\.s.h.e.l.l.3.2...d.l.l.,.-.2.1.7.8.1.\r.\n.""
[*] binary.imports.imports.lib: ""
[*] binary.imports.imports.names: ""

[*] filename: "desktop.ini"
[*] hashes.md5: "5b8a2ba3138573583ff9e0158096ec48"
[*] entropy: 3.5208218
[*] binary.sections.sections.name: ""
[*] binary.sections.sections.entropy: ""
[*] ads.name: """"
[*] ads.bytes: "174"
[*] ads.first_256_bytes: ""??????\r.\n.[...S.h.e.l.l.C.l.a.s.s.I.n.f.o.].\r.\n.L.o.c.a.l.i.z.e.d.R.e.s.o.u.r.c.e.N.a.m.e.=.@.%.S.y.s.t.e.m.R.o.o.t.%.\\.s.y.s.t.e.m.3.2.\\.s.h.e.l.l.3.2...d.l.l.,.-.2.1.8.1.7.\r.\n.""
[*] binary.imports.imports.lib: ""
[*] binary.imports.imports.names: ""

[*] filename: "RunAsService.exe"
[*] hashes.md5: "4b92bd03d0c1e1f793ed1b499534211b"
[*] entropy: 4.623817
[*] binary.sections.sections.name: "".text"\n".rsrc"\n".reloc""
[*] binary.sections.sections.entropy: "4.7316236\n4.3263397\n0.081539415"
[*] ads.name: """\n"evil"\n"SmartScreen"\n"Zone.Identifier""
[*] ads.bytes: "23552\n34\n7\n123"
[*] ads.first_256_bytes: ""MZ???.\u0003...\u0004...??????..???.......@...................................???...\u000e\u001f???\u000e.???\t???!???\u0001L???!This program cannot be run in DOS mode.\r\r\n$.......PE..L\u0001\u0003.B??????Y........???.\u0002\u0001\u000b\u00010..P...\n......???o... ...???....@.. ...\u0002..\u0004.......\u0004........???...\u0002......\u0003.@???..\u0010..\u0010....\u0010..\u0010......\u0010.........."\n"\"this is hiding info in an ADS\" \r\n"\n"Anaheim"\n"[ZoneTransfer]\r\nZoneId=3\r\nReferrerUrl=http://runasservice.com/\r\nHostUrl=http://runasservice.com/Download/RunAsService.exe\r\n""
[*] binary.imports.imports.lib: ""mscoree.dll""
[*] binary.imports.imports.names: "["_CorExeMain"]"
```
#### Print a dot delimited list of all key names
```
cat .\res.txt | .\jve --unique
ads
ads.bytes
ads.first_256_bytes
ads.name
binary
binary.entry_point
binary.exports
binary.exports.count
binary.exports.hashes
binary.exports.hashes.md5
binary.exports.hashes.ssdeep
binary.exports.names
binary.imports
binary.imports.func_count
binary.imports.hashes
binary.imports.hashes.md5
binary.imports.hashes.md5_sorted
binary.imports.hashes.ssdeep
binary.imports.hashes.ssdeep_sorted
binary.imports.imports
binary.imports.imports.count
binary.imports.imports.lib
binary.imports.imports.names
binary.imports.imports.names.info
binary.imports.imports.names.more_interesting
binary.imports.imports.names.name
binary.imports.lib_count
binary.is_64
binary.is_dotnet
binary.is_lib
binary.linker
binary.linker.major_version
binary.linker.minor_version
binary.pe_info
binary.pe_info.company_name
binary.pe_info.file_description
binary.pe_info.file_version
binary.pe_info.internal_name
binary.pe_info.legal_copyright
binary.pe_info.original_filename
binary.pe_info.product_name
binary.pe_info.product_version
binary.sections
binary.sections.sections
binary.sections.sections.entropy
binary.sections.sections.md5
binary.sections.sections.name
binary.sections.sections.raw_size
binary.sections.sections.ssdeep
binary.sections.sections.virt_address
binary.sections.sections.virt_size
binary.sections.total_raw_bytes
binary.sections.total_sections
binary.sections.total_virt_bytes
binary.timestamps
binary.timestamps.compile
binary.timestamps.debug
bytes
directory
entropy
extension
filename
hashes
hashes.md5
hashes.sha1
hashes.sha256
hashes.ssdeep
is_hidden
is_link
link
link.abs_path
link.arguments
link.comment
link.drive_serial_number
link.drive_type
link.flags
link.hotkey
link.icon_location
link.rel_path
link.show_command
link.volume_label
link.working_dir
mime_type
path
runtime_env
runtime_env.device_type
runtime_env.run_as_admin
runtime_env.timestamp
strings
timestamps
timestamps.access_fn
timestamps.access_si
timestamps.create_fn
timestamps.create_si
timestamps.mft_record
timestamps.modify_fn
timestamps.modify_si
```
#### Example log parsed by JVE -> using the [File Meta Data tool](https://github.com/theflakes/fmd)
```
{
  "runtime_env": {
    "timestamp": "2023-05-02T00:42:56.293126300+00:00",
    "device_type": "Windows 10.0.22621 (Workstation)",
    "run_as_admin": false
  },
  "path": "C:\\Users\\thefl\\code\\jve\\target\\release\\fmd.exe",
  "directory": "C:\\Users\\thefl\\code\\jve\\target\\release",
  "filename": "fmd.exe",
  "extension": "exe",
  "bytes": 912384,
  "mime_type": "application/x-executable",
  "is_hidden": false,
  "is_link": false,
  "link": {
    "rel_path": "",
    "abs_path": "",
    "arguments": "",
    "working_dir": "",
    "icon_location": "",
    "hotkey": "",
    "comment": "",
    "show_command": "",
    "flags": "",
    "drive_type": "",
    "drive_serial_number": "",
    "volume_label": ""
  },
  "timestamps": {
    "access_fn": "",
    "access_si": "2023-05-02T00:42:56.251",
    "create_fn": "",
    "create_si": "2023-04-21T20:45:02.519",
    "modify_fn": "",
    "modify_si": "2023-05-01T21:43:31.397",
    "mft_record": ""
  },
  "entropy": 6.361143,
  "hashes": {
    "md5": "2ecfb9be3cbe6cd13ef8c277a5b820ce",
    "sha1": "d9803b3e61857c87f76901429e1c142afb98b9ac",
    "sha256": "2085fc3f76dea5d4841bf32850abb9d5146494b93b3fbf6bdc5012170165f022",
    "ssdeep": "12288:fD2qMN6ONCPoXU53OsbaROCOZEHmj2igLQ71cJtps:fD0NCUW3OsiCZamjN97uJtu"
  },
  "ads": [],
  "binary": {
    "is_64": true,
    "is_dotnet": false,
    "is_lib": false,
    "entry_point": "0x8783c",
    "pe_info": {
      "product_version": "",
      "original_filename": "",
      "file_description": "",
      "file_version": "",
      "product_name": "",
      "company_name": "",
      "internal_name": "",
      "legal_copyright": ""
    },
    "timestamps": {
      "compile": "2023-05-01T21:43:31",
      "debug": "2023-05-01T21:43:31"
    },
    "linker": {
      "major_version": 14,
      "minor_version": 35
    },
    "sections": {
      "total_sections": 6,
      "total_raw_bytes": 911360,
      "total_virt_bytes": 914764,
      "sections": [
        {
          "name": ".text",
          "entropy": 6.26124,
          "md5": "81375221719cb4d50742bcd1f973c2ca",
          "ssdeep": "12288:yD2qMN6ONCPoXU53OsbaROCOZEHmj2igLQ71cJt:yD0NCUW3OsiCZamjN97uJt",
          "virt_address": "0x1000",
          "raw_size": 629760,
          "virt_size": 629584
        },
        {
          "name": ".rdata",
          "entropy": 5.6207933,
          "md5": "2c0e17c0685a0e6656adcd4a74349126",
          "ssdeep": "3072:4t6vBqobiVcZaYM+qVmuorUIbpKFMLkt8q1uChX0aUUCLeV:CN0cs4IbsPPWS",
          "virt_address": "0x9b000",
          "raw_size": 254976,
          "virt_size": 254608
        },
        {
          "name": ".data",
          "entropy": 2.0772414,
          "md5": "76ed25a79149094d9290fd8060ded18e",
          "ssdeep": "24:c1Bf6uSkeKP6uSkeK8hBSqxSSSS4SwVVCVou:IBTk4TkPkiSSSSSHCVou",
          "virt_address": "0xda000",
          "raw_size": 3072,
          "virt_size": 8024
        },
        {
          "name": ".pdata",
          "entropy": 5.764084,
          "md5": "56119c060db9396f7dfa4f1b92895654",
          "ssdeep": "384:ko2UXFrDBr2gqjmD3t8bmbB9x9hvNoPMLKMwJ5EoRrLG/lYRnak02:VtDImxLbDnboPuKDJiivOlYRna5",
          "virt_address": "0xdc000",
          "raw_size": 16384,
          "virt_size": 16008
        },
        {
          "name": "_RDATA",
          "entropy": 3.3046613,
          "md5": "31ff6f2798d8f7c00aaf516b84718be2",
          "ssdeep": "6:P/hxYw51Uoit95idqOJMYwCTA4Fbb3zyveNA4XK13H:If6PCYo4FbKH",
          "virt_address": "0xe0000",
          "raw_size": 512,
          "virt_size": 348
        },
        {
          "name": ".reloc",
          "entropy": 5.332825,
          "md5": "e766cc951570837d276bb7ff2aca00e4",
          "ssdeep": "192:8Qn81cD1c+hvJeOV1LWgtQ0Mq9ucgssoEX:8Qn8KBBeOVhrtQPq9wssoE",
          "virt_address": "0xe1000",
          "raw_size": 6656,
          "virt_size": 6192
        }
      ]
    },
    "imports": {
      "hashes": {
        "md5": "ad3f2eabfdf67bac7ed8a69a4c402917",
        "md5_sorted": "5919e44bd5534590d79649bcc72515fc",
        "ssdeep": "48:pErXcdf/p9zWwTxrWA1stv4Bc+pRl7EcbfK:arXcV/pJWwTxrWA1stv4Bc+pRrS",
        "ssdeep_sorted": "48:mbfKW5W6yFQCg9/w3+nmPc1hnxQsGvXHcvB:UCW5W6YQCg5Rnm8hnxQsGvXHcvB"
      },
      "lib_count": 3,
      "func_count": 105,
      "imports": [
        {
          "lib": "KERNEL32.dll",
          "count": 101,
          "names": [
            {
              "name": "CloseHandle",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCurrentProcess",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetFilePointerEx",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetLastError",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FindFirstFileW",
              "more_interesting": true,
              "info": "Searches a directory for a file or subdirectory with a name."
            },
            {
              "name": "FindClose",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCommandLineW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetLastError",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetModuleFileNameW",
              "more_interesting": true,
              "info": "Retrieves the fully qualified path for the file that contains the specified module."
            },
            {
              "name": "AddVectoredExceptionHandler",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetThreadStackGuarantee",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCurrentThread",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "HeapReAlloc",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FileTimeToSystemTime",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SystemTimeToTzSpecificLocalTime",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SystemTimeToFileTime",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetTimeZoneInformation",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "HeapAlloc",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetProcessHeap",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "Sleep",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetModuleHandleA",
              "more_interesting": true,
              "info": "Retrieves a module handle for the specified module."
            },
            {
              "name": "TryAcquireSRWLockExclusive",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "ReleaseSRWLockExclusive",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetStdHandle",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetConsoleMode",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FreeLibrary",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "MultiByteToWideChar",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "WriteConsoleW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCurrentDirectoryW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "WaitForSingleObjectEx",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "LoadLibraryA",
              "more_interesting": true,
              "info": "Loads the specified module into the address space of the calling process."
            },
            {
              "name": "CreateMutexA",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "ReleaseMutex",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RtlLookupFunctionEntry",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetModuleHandleW",
              "more_interesting": true,
              "info": "Retrieves a module handle for the specified module."
            },
            {
              "name": "FormatMessageW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "CreateFileW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetFileInformationByHandle",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetFileInformationByHandleEx",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetFullPathNameW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FindNextFileW",
              "more_interesting": true,
              "info": "Continues a file search for a previous call to the 'findfirstfile/findfirstfileex/findfirstfiletransacted' function."
            },
            {
              "name": "AcquireSRWLockExclusive",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "ExitProcess",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "QueryPerformanceCounter",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "QueryPerformanceFrequency",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetSystemTimeAsFileTime",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RtlCaptureContext",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "AcquireSRWLockShared",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "ReleaseSRWLockShared",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetEnvironmentVariableW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetFinalPathNameByHandleW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetProcAddress",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "LoadLibraryExW",
              "more_interesting": true,
              "info": "Loads the specified module into the address space of the calling process."
            },
            {
              "name": "WaitForSingleObject",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "HeapFree",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCurrentProcessId",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCurrentThreadId",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "InitializeSListHead",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RtlVirtualUnwind",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "IsDebuggerPresent",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "UnhandledExceptionFilter",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetUnhandledExceptionFilter",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetStartupInfoW",
              "more_interesting": true,
              "info": "Retrieves the contents of the STARTUPINFO structure that was specified when the calling process was created."
            },
            {
              "name": "IsProcessorFeaturePresent",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RtlUnwindEx",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "EncodePointer",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RaiseException",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "EnterCriticalSection",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "LeaveCriticalSection",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "DeleteCriticalSection",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "InitializeCriticalSectionAndSpinCount",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "TlsAlloc",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "TlsGetValue",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "TlsSetValue",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "TlsFree",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "RtlPcToFileHeader",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "WriteFile",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "TerminateProcess",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetModuleHandleExW",
              "more_interesting": true,
              "info": "Retrieves a module handle for the specified module and increments the module's reference count."
            },
            {
              "name": "GetCommandLineA",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FindFirstFileExW",
              "more_interesting": true,
              "info": "Searches a directory for a file or subdirectory with a name."
            },
            {
              "name": "IsValidCodePage",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetACP",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetOEMCP",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetCPInfo",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "WideCharToMultiByte",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetEnvironmentStringsW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FreeEnvironmentStringsW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetEnvironmentVariableW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SetStdHandle",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetFileType",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetStringTypeW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FlsAlloc",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FlsGetValue",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FlsSetValue",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FlsFree",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "CompareStringW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "LCMapStringW",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "HeapSize",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "FlushFileBuffers",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "GetConsoleOutputCP",
              "more_interesting": false,
              "info": ""
            }
          ]
        },
        {
          "lib": "ADVAPI32.dll",
          "count": 3,
          "names": [
            {
              "name": "GetTokenInformation",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "OpenProcessToken",
              "more_interesting": false,
              "info": ""
            },
            {
              "name": "SystemFunction036",
              "more_interesting": false,
              "info": ""
            }
          ]
        },
        {
          "lib": "bcrypt.dll",
          "count": 1,
          "names": [
            {
              "name": "BCryptGenRandom",
              "more_interesting": false,
              "info": ""
            }
          ]
        }
      ]
    },
    "exports": {
      "hashes": {
        "md5": "d41d8cd98f00b204e9800998ecf8427e",
        "ssdeep": "3::"
      },
      "count": 0,
      "names": []
    }
  },
  "strings": []
}
```