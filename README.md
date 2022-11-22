![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/sv_extractor)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/sv_extractor)
![GitHub stars](https://img.shields.io/github/stars/CMIW/sv_extractor?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/sv_extractor?style=social)

# sv_extractor

This tool is based on [SCVI_Extract](https://github.com/psthrn42/SCVI_Extract). This tool was created to make extracting data from SV binary files easier by making an executable CLI instead of a python script. It should also support both windows and linux.

Right now, for the .trpak files, it can only unpack the files without oodle lz compression. To decompress those a windows only dll is needed. For that [SCVI_Extract](https://github.com/psthrn42/SCVI_Extract) works better but still is windows only. Soon will base the project on [melsbacksfriend/SCVI_Extract](https://github.com/melsbacksfriend/SCVI_Extract/tree/ec39739079d82284b39f57bc5b4849558567a9ed) to add linux and windows support.

## Requirements

0. ['flatc'](https://github.com/google/flatbuffers/releases) for linux or ['flatc.exe'](https://github.com/google/flatbuffers/releases) for windows
<!-- 1. 'oo2core_6_win64.dll' for windows or 'liblinoodle.so' -->

## How to use the tool

0. Download the binaries for your system from the [latest release](https://github.com/CMIW/sv_extractor/releases/latest). 
1. Extract the binaries from the zip file.
2. Place the files ['flatc.exe'](https://github.com/google/flatbuffers/releases)<!--, 'oo2core_6_win64.dll' --> (for windows) or ['flatc'](https://github.com/google/flatbuffers/releases) (for linux) in the [bin](/bin) folder.
3. Open a terminal where you have the binaries and run the following command:

Linux example for trpfs:
```
./sv_extractor --romfs <path to romfs folder> --output <path> --extraction <extraction option>
```

Windows example for trpfs:
```
sv_extractor.exe --romfs <path to romfs folder> --output <path> --extraction <extraction option>
```

## Extraction options
0. trpfs
1. trpak
2. full