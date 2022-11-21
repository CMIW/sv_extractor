![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/sv_extractor)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/sv_extractor)
![GitHub stars](https://img.shields.io/github/stars/CMIW/sv_extractor?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/sv_extractor?style=social)

# sv_extractor

This tool is based on [SCVI_Extract](https://github.com/psthrn42/SCVI_Extract). This tool was created to make extracting data from SV binary files easier by making an executable CLI instead of a python script. It should also support both windows and linux.

## Requirements

0. ['flatc.exe'](https://github.com/google/flatbuffers/releases) and 'oo2core_6_win64.dll' for windows
1. ['flatc'](https://github.com/google/flatbuffers/releases) for linux

## How to use the tool

0. Download the binaries for your system from the [latest release](https://github.com/CMIW/sv_extractor/releases). 
1. Extract the binaries from the zip file.
2. Place the files ['flatc.exe'](https://github.com/google/flatbuffers/releases), 'oo2core_6_win64.dll' (for windows) and ['flatc'](https://github.com/google/flatbuffers/releases) (for linux) in the [bin](/bin) folder.
3. Open a terminal where you have the binaries and run the following command:

Linux example:
```
./sv_extraor --romfs <path to romfs folder> --output <path> --extraction <extraction option>
```

Linux example:
```
sv_extraor.exe --romfs <path to romfs folder> --output <path> --extraction <extraction option>
```

## Extraction options
0. trpfs