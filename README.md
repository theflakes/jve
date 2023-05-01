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

"06e8f7e6ddd666dbd323f7d9210f91ae"
"089d48a11bff0df720f1079f5dc58a83"
"08c99274edaa8548232f0498fec0bcf4"
"0e54be9771282f8eaa026b967006f41b"
"0fa26b6c98419b5e7c00efffb5835612"
"2246f34c9f9cbc23697d4498391c19a5"
"2ddd5dbcb911bcfd13123aedb84584be"
"3a361ad38db9afe1997177a290607d22"
"3a37312509712d4e12d27240137ff377"
"3a6ccb171c990074990a2dd10e6014c0"
"3b960da228cc489b622697659c885d64"
"40755e75c0a31ed4c87fea0456c38b27"
"420ae295c6a48e05bbe7feb6e4f5e1b1"
"449f2e76e519890a212814d96ce67d64"
"50a956778107a4272aae83c86ece77cb"
"59071590099d21dd439896592338bf95"
"5d42dddda9951546c9d43f0062c94d39"
"614d318c54542cd30a472965ac2e3c6e"
"6576cae679fffc7f8f296fe9a1b3f867"
"6fc234ad3752e1267b34fb12bcd6718b"
"881dfac93652edb0a8228029ba92d0f5"
"a1bce84d1baa50b883a8aaf0fb154d6d"
"b441cf59b5a64f74ac3bed45be9fadfc"
"b6acbeb59959aa5412a7565423ea7bab"
"bafb57f3a2d653461cde7956e6ace690"
"cd9d358a1c3e634f13f7fde433feb483"
"cda629e9edf62c0efb17fbea1e32dad0"
"cdc02ae6a5852cf0bcfdea4544ebdcab"
"d30f32128c338c3a6ca05ff5ae7a4b7f"
"d41d8cd98f00b204e9800998ecf8427e"
```
#### Example output parsing a JSON nested structure a common field across all JSON logs
```
cat .\files.json | .\jve --unique --values --key "hashes"

{"md5":"06e8f7e6ddd666dbd323f7d9210f91ae","sha1":"883ae527ee83ed9346cd82c33dfc0eb97298dc14","sha256":"8301e344371b0753d547b429c5fe513908b1c9813144f08549563ac7f4d7da68","ssdeep":"12:QZsiL5wmHOlDmo0qml3lDmo0qmZclLwr2FlDmo0IWUol94klrgl2FlDmo0qjKAZY:QCGwv4o0x34o02lLwiF4o0ZvbUsF4o0Z"}
{"md5":"089d48a11bff0df720f1079f5dc58a83","sha1":"88f1c647378b5b22ebadb465dc80fcfd9e7b97c9","sha256":"a9e8ad0792b546a4a8ce49eda82b327ad9581141312efec3ac6f2d3ad5a05f17","ssdeep":"12:QZsiL5wmHOlDmo0qmEclLwr2FlDmo0IWhvXiTpKUAa0C6wyEZwyEG:QCGwv4o0RlLwiF4o0hX+wDXZWX"}
{"md5":"08c99274edaa8548232f0498fec0bcf4","sha1":"9535e4ebce9ec86680fc4fc782dda211b5d14427","sha256":"9ff7cf330c230565d792c7d3316352d018f6b403fc0f5c82f32419a4d7d38bd1","ssdeep":"3:YCQta6Se+RfE9qHgJHqSJn:YCQta5VsoHgJqSJ"}
{"md5":"0e54be9771282f8eaa026b967006f41b","sha1":"fac9b46269c4e9c323ff9b0f6c2c32b94300f901","sha256":"271458a2755e266a0c86b0ff4fee20a2f58f99c44c88e4ba51eb6a58105b2e36","ssdeep":"12:8mNgXccldu/UbdpYuYRfzpewmJ5uSc3AtbOwYRfzpewm:8EgXflAAd2Jzswm6Scwt2Jzswm"}
{"md5":"0fa26b6c98419b5e7c00efffb5835612","sha1":"d904d6683a548b03950d94da33cdfccbb55a9bc7","sha256":"4094d158e3b0581ba433a46d0dce62f99d8c0fd1b50bb4d0517ddc0a4a1fde24","ssdeep":"6:TMV08iTRH/iNBKNxG+KNhkF2deqYutDSA8UcXq2SUVrj:TMG8ip/ifO8+OhkMQqYaOA8UnHUVrj"}
{"md5":"2246f34c9f9cbc23697d4498391c19a5","sha1":"66257d16876f175e307f336d876f4643ea320152","sha256":"3780593830750933d4b8da35bf02cbe2c31eb8f96ab970db9d350f02f68ad0cd","ssdeep":"12:4cW/HUKspeRY7xNhaKzM/79yo+ermAs40slEAqyaZVKo:RGHUKAgYdaKz87soxrr0qEAqnZVKo"}
{"md5":"2ddd5dbcb911bcfd13123aedb84584be","sha1":"15fa243e48da777ee4fb69d50f8c269dff7134be","sha256":"ea1ba269f9edf809419a55e34ea454d745201fa62124dace4b0663f9bc6c7d91","ssdeep":"12:jLZRg/rmZbyz1eLXrE8PjMAlj1zqBJAyRAmXWjAPJe69AZB85pCuygk14GuybgQx:jsmsz1AXrMY1XyPWjeJeT6pC+GuyQI"}
{"md5":"3a361ad38db9afe1997177a290607d22","sha1":"38af4963f157f0df31b4c38f0ae859ec94202cc4","sha256":"06a4c15b779e0d06c8b8981cfca14cb7bbff55781818c9e2f9a4353fd8140c13","ssdeep":"12:8bbwaZkKF+QbwaZ9YjAF4FgNNlUCfJIWD4t2YlYM/OqCQX8KzLbXkQ/V4VQE5meW:8bMapMa8AF4ejlfJz/M/zvXD/K55meN"}
{"md5":"3a37312509712d4e12d27240137ff377","sha1":"30ced927e23b584725cf16351394175a6d2a9577","sha256":"b029393ea7b7cf644fb1c9f984f57c1980077562ee2e15d0ffd049c4c48098d3","ssdeep":"6:QyqRsioTA5wmHOlRaQmZWGokJqAMhAlt4DAlLwkAl2FlRaQmZWGokJISlVl9:QZsiL5wmHOlDmo0qmt4clLwr2FlDmo0d"}
{"md5":"3a6ccb171c990074990a2dd10e6014c0","sha1":"9ca2e6a0b081dc289cc8dafa779f5928685dabb9","sha256":"6fbc71117b289d072b90f304e631b85ebd974871f92cfc5318c5139ad512f594","ssdeep":"48:Lr99rPZykNzqnLdDeB80T1Sba8fdixsYSE+so4hC7X9G9ajOmth18g3hA2ULjW:Lr99rZnLBVTf8fdixszsoN518g3hILjW"}
{"md5":"3b960da228cc489b622697659c885d64","sha1":"00686a12f1a43501f6eea2140da9be141a11bd3b","sha256":"a4234e2cf44c57609fd7cb0f9f0a33ee136b542fba5121ac02d85b38fb2ea02d","ssdeep":"12:QZsiL5wmHOlDmo0qmC6clLwr2FlDmo0IWZS8s+iTpKUdDsWIstn:QCGwv4o04lLwiF4o03+w4Ian"}
{"md5":"40755e75c0a31ed4c87fea0456c38b27","sha1":"ce5b69bddabc92cc7cebd116964785184cb8c403","sha256":"bf695891725540167c7db0bbf652468dd6139c61361102a8a5cf10111a4c6377","ssdeep":"48:k/8kwxoBytpFuVb2V3sHwxoBytLFuVb2V:kvwxsytpFuVb2V3EwxsytLFuVb2V"}
{"md5":"420ae295c6a48e05bbe7feb6e4f5e1b1","sha1":"b63c804e7e66e2edadc5700e0216b1b7fff7f716","sha256":"0af3ad72af17e1321674025bd8e8e971c16b0d92b6d302c080a1f57661615b14","ssdeep":"6:k3fcPmIpdOMkZpW2RXQ/pMAooJPhZYaDxZSJzXxQKcRSXmRsiAuK:kUPxWG2exBb5DlZ+jxQh2mRsUK"}
{"md5":"449f2e76e519890a212814d96ce67d64","sha1":"a316a38e1a8325bef6f68f18bc967b9aaa8b6ebd","sha256":"48a6703a09f1197ee85208d5821032b77d20b3368c6b4de890c44fb482149cf7","ssdeep":"12:QZsiL5wmHyL0bO4fgL0bO40clLwr2FlDmo0IWdY:QCGwFgAgdlLwiF4o01Y"}
{"md5":"50a956778107a4272aae83c86ece77cb","sha1":"10bce7ea45077c0baab055e0602eef787dba735e","sha256":"b287b639f6edd612f414caf000c12ba0555adb3a2643230cbdd5af4053284978","ssdeep":"12:QZsiL5wmHOlDmo0qmclDmo0qmJclLwr2FlDmo0IWVvklrgl2FlDmo0qjKArn:QCGwv4o0o4o0mlLwiF4o090UsF4o01Ar"}
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
binary.entry_point
binary.exports.count
binary.exports.hashes.md5
binary.exports.hashes.ssdeep
binary.exports.names
binary.imports.func_count
binary.imports.hashes.md5
binary.imports.hashes.md5_sorted
binary.imports.hashes.ssdeep
binary.imports.hashes.ssdeep_sorted
binary.imports.imports
binary.imports.lib_count
binary.is_64
binary.is_dotnet
binary.is_lib
binary.linker.major_version
binary.linker.minor_version
binary.pe_info.company_name
binary.pe_info.file_description
binary.pe_info.file_version
binary.pe_info.internal_name
binary.pe_info.legal_copyright
binary.pe_info.original_filename
binary.pe_info.product_name
binary.pe_info.product_version
binary.sections.sections
binary.sections.total_raw_bytes
binary.sections.total_sections
binary.sections.total_virt_bytes
binary.timestamps.compile
binary.timestamps.debug
bytes
directory
entropy
extension
filename
hashes.md5
hashes.sha1
hashes.sha256
hashes.ssdeep
is_hidden
is_link
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
runtime_env.device_type
runtime_env.run_as_admin
runtime_env.timestamp
strings
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
    "timestamp": "2023-01-29T14:05:44.595674700+00:00",
    "device_type": "Windows 10.0.22621 (Workstation)",
    "run_as_admin": true
  },
  "path": "C:\\Users\\thefl\\code\\jve\\target\\release\\fmd.exe",
  "directory": "C:\\Users\\thefl\\code\\jve\\target\\release",
  "filename": "fmd.exe",
  "extension": "exe",
  "bytes": 858624,
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
    "access_fn": "2023-01-27T02:25:30.097",
    "access_si": "2023-01-29T14:05:43.754",
    "create_fn": "2023-01-23T23:31:06.203",
    "create_si": "2023-01-23T23:31:06.203",
    "modify_fn": "2023-01-27T02:25:30.097",
    "modify_si": "2023-01-27T02:19:00.372",
    "mft_record": "2023-01-27T02:25:30.097"
  },
  "entropy": 6.3832226,
  "hashes": {
    "md5": "729e4a560c865f7cc28725337abcb4a0",
    "sha1": "8df4ca63445baac454f17aa773ad8f21d8e72abf",
    "sha256": "3da337a3655a4188166df7f3def588336c95d1f9647bb75453a743e0679e357c",
    "ssdeep": "12288:vWXmv1C2Z0KDoNqHbyamXe0Fuo1eGyjq3H/YYEb:/v1CPViKe0FuoL3fYr"
  },
  "ads": [
    {
      "name": "",
      "bytes": 858624,
      "first_256_bytes": "MZ�.\u0003...\u0004...��..�.......@...................................�...\u000e\u001f�\u000e.�\t�!�\u0001L�!This program cannot be run in DOS mode.\r\r\n$.......w\f��3m�3m�3m��\u001f�:m��\u001fὺm��\u001f�9m�|\u0011�\u001am�|\u0011�#m�|\u0011�:m��\u001f�4m�3m�Km�3m�1m��\u0011�2m�Rich3m�................PE..d�\u0006"
    }
  ],
  "binary": {
    "is_64": true,
    "is_dotnet": false,
    "is_lib": false,
    "entry_point": "0x7d13c",
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
      "compile": "2023-01-27T02:19:00",
      "debug": "2023-01-27T02:19:00"
    },
    "linker": {
      "major_version": 14,
      "minor_version": 34
    },
    "sections": {
      "total_sections": 6,
      "total_raw_bytes": 857600,
      "total_virt_bytes": 861830,
      "sections": [
        {
          "name": ".text",
          "entropy": 6.2971563,
          "md5": "c99ce7ebf9c3d7c97d2cd6a12ec1f0c1",
          "ssdeep": "12288:kWXmv1C2Z0KDoNqHbyamXe0Fuo1eGyjq3H/Y:Qv1CPViKe0FuoL3fY",
          "virt_address": "0x1000",
          "raw_size": 586240,
          "virt_size": 585888
        },
        {
          "name": ".rdata",
          "entropy": 5.5931087,
          "md5": "a16b8a37ec44c6deda1a0f054e462b8a",
          "ssdeep": "3072:W7PBgorVcZjHFOyWGXD1Sz30Kf5Lkt8X1iCc8QRCse4iDU:Qv0cz3ISTcmL",
          "virt_address": "0x91000",
          "raw_size": 245760,
          "virt_size": 245722
        },
        {
          "name": ".data",
          "entropy": 2.0857084,
          "md5": "016a5eccbd53c1c50e89b6df6eb7bb89",
          "ssdeep": "24:rJU2XwBf6uSkeKP6uSkeK8hBSqW////70VVVVVVVVCVrn1+geu:rPwBTk4TkPkLrrn8geu",
          "virt_address": "0xcd000",
          "raw_size": 3072,
          "virt_size": 7976
        },
        {
          "name": ".pdata",
          "entropy": 5.816629,
          "md5": "52d2e6aa0d45d23c0faf841a2e32cc87",
          "ssdeep": "384:e7s1X/jCknoxmbZ2CVPxKnTBMaIDzCUc+stF+eW6Tz:egtZnUhC9QT1+zPr0F9WA",
          "virt_address": "0xcf000",
          "raw_size": 15872,
          "virt_size": 15804
        },
        {
          "name": "_RDATA",
          "entropy": 3.3070078,
          "md5": "725c7ab051d81cad419fe04eea5bbf37",
          "ssdeep": "6:pB8GR/iLSQYIkINDezUILFGYl1Gy6338mXlzuimaAR4WJqh6It3leMyaG4IsFzqc:bRqLlez574DsRaReMyRjCYmDl",
          "virt_address": "0xd3000",
          "raw_size": 512,
          "virt_size": 348
        },
        {
          "name": ".reloc",
          "entropy": 5.4327927,
          "md5": "bc62a6722cef4cab1e0bbce63669684e",
          "ssdeep": "96:vSsDvDfKenEvm1mPxBf3Cpa8Xly3PdcRpHnCfln3FhQdnnZnvnU0kvP8vvSv0PEX:LDCe2NJN3CptY/dm5nUn3FhQdZP1ssil",
          "virt_address": "0xd4000",
          "raw_size": 6144,
          "virt_size": 6092
        }
      ]
    },
    "imports": {
      "hashes": {
        "md5": "87f23a29b12b656687833c7272d2fe90",
        "md5_sorted": "201ae632e017b2e38981f7ba23a38976",
        "ssdeep": "48:URErX0dap9zWwTxrWp1stv4Bc+pRlKEfKh/7R7K:UCrX0opJWwTxrWp1stv4Bc+pRXINW",
        "ssdeep_sorted": "48:mb7R7g/9W5W6yFQCg9/w3+nmPc1hnxQsGvXHcvB:UN+9W5W6YQCg5Rnm8hnxQsGvXHcvB"
      },
      "lib_count": 3,
      "func_count": 106,
      "imports": [
        {
          "lib": "KERNEL32.dll",
          "count": 101,
          "names": [
            "FreeLibrary",
            "CloseHandle",
            "GetCurrentProcess",
            "SetFilePointerEx",
            "GetLastError",
            "FindFirstFileW",
            "FindClose",
            "GetCommandLineW",
            "SetLastError",
            "GetModuleFileNameW",
            "AddVectoredExceptionHandler",
            "SetThreadStackGuarantee",
            "GetCurrentThread",
            "HeapAlloc",
            "GetProcessHeap",
            "HeapReAlloc",
            "FileTimeToSystemTime",
            "SystemTimeToTzSpecificLocalTime",
            "SystemTimeToFileTime",
            "GetTimeZoneInformation",
            "Sleep",
            "GetModuleHandleA",
            "TryAcquireSRWLockExclusive",
            "ReleaseSRWLockExclusive",
            "GetStdHandle",
            "GetProcAddress",
            "WaitForSingleObject",
            "WriteConsoleW",
            "GetCurrentDirectoryW",
            "WaitForSingleObjectEx",
            "LoadLibraryA",
            "CreateMutexA",
            "ReleaseMutex",
            "RtlLookupFunctionEntry",
            "GetModuleHandleW",
            "FormatMessageW",
            "CreateFileW",
            "GetFileInformationByHandle",
            "GetFileInformationByHandleEx",
            "GetFullPathNameW",
            "FindNextFileW",
            "AcquireSRWLockExclusive",
            "ExitProcess",
            "QueryPerformanceCounter",
            "QueryPerformanceFrequency",
            "GetSystemTimeAsFileTime",
            "RtlCaptureContext",
            "AcquireSRWLockShared",
            "ReleaseSRWLockShared",
            "GetEnvironmentVariableW",
            "GetFinalPathNameByHandleW",
            "LoadLibraryExW",
            "GetConsoleMode",
            "HeapFree",
            "GetConsoleOutputCP",
            "FlushFileBuffers",
            "GetCurrentProcessId",
            "GetCurrentThreadId",
            "InitializeSListHead",
            "RtlVirtualUnwind",
            "IsDebuggerPresent",
            "UnhandledExceptionFilter",
            "SetUnhandledExceptionFilter",
            "GetStartupInfoW",
            "IsProcessorFeaturePresent",
            "RtlUnwindEx",
            "EncodePointer",
            "RaiseException",
            "EnterCriticalSection",
            "LeaveCriticalSection",
            "DeleteCriticalSection",
            "InitializeCriticalSectionAndSpinCount",
            "TlsAlloc",
            "TlsGetValue",
            "TlsSetValue",
            "TlsFree",
            "RtlPcToFileHeader",
            "WriteFile",
            "TerminateProcess",
            "GetModuleHandleExW",
            "GetCommandLineA",
            "FindFirstFileExW",
            "IsValidCodePage",
            "GetACP",
            "GetOEMCP",
            "GetCPInfo",
            "MultiByteToWideChar",
            "WideCharToMultiByte",
            "GetEnvironmentStringsW",
            "FreeEnvironmentStringsW",
            "SetEnvironmentVariableW",
            "SetStdHandle",
            "GetFileType",
            "GetStringTypeW",
            "FlsAlloc",
            "FlsGetValue",
            "FlsSetValue",
            "FlsFree",
            "CompareStringW",
            "LCMapStringW",
            "HeapSize"
          ]
        },
        {
          "lib": "ADVAPI32.dll",
          "count": 2,
          "names": [
            "OpenProcessToken",
            "GetTokenInformation"
          ]
        },
        {
          "lib": "bcrypt.dll",
          "count": 3,
          "names": [
            "BCryptOpenAlgorithmProvider",
            "BCryptCloseAlgorithmProvider",
            "BCryptGenRandom"
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