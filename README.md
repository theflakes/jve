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
    -d, --delimiter ","           Value to use to seperate field value output
    -f, --fields "a.b.c.d,a.b.e"  Comma seperated list of fields in dot notation

NOTE:   If a field is an array or the field name occurs in an array, 
        this program will concatinate all array field values into a comma 
        seperated quoted string across all array elements.
```
#### Example Output
```
fmd.exe .\fmd.exe | jve -d "," -f "filename,hashes.md5,entropy,binary.sections.sections.name,binary.sections.sections.entropy,binary.imports.imports.lib,binary.imports.imports.count"

"fmd.exe","729e4a560c865f7cc28725337abcb4a0",6.3832226,"".text",".rdata",".data",".pdata","_RDATA",".reloc"","6.2971563,5.5931087,2.0857084,5.816629,3.3070078,5.4327927",""KERNEL32.dll","ADVAPI32.dll","bcrypt.dll"","101,2,3"
```
#### Log for above output
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