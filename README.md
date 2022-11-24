![GitHub repo size](https://img.shields.io/github/repo-size/CMIW/sv_extractor)
![GitHub contributors](https://img.shields.io/github/contributors/CMIW/sv_extractor)
![GitHub stars](https://img.shields.io/github/stars/CMIW/sv_extractor?style=social)
![GitHub forks](https://img.shields.io/github/forks/CMIW/sv_extractor?style=social)

# sv_extractor

This tool is based on [SCVI_Extract](https://github.com/psthrn42/SCVI_Extract). This tool was created to make extracting data from SV binary files easier by making an executable CLI instead of a python script. It should also support both windows and linux.

## Requirements

None, just download the binaries for your system from the [latest release](https://github.com/CMIW/sv_extractor/releases/latest).

## How to use the tool

0. Download the binaries for your system from the [latest release](https://github.com/CMIW/sv_extractor/releases/latest). 
1. Extract the binaries from the zip file.
2. Place the files ['flatc.exe'](https://github.com/google/flatbuffers/releases) (for windows) or ['flatc'](https://github.com/google/flatbuffers/releases) (for linux) in the [bin](/bin) folder.
3. Open a terminal where you have the binaries and run the following command:

Linux example for trpfs:
```
./sv_extractor --romfs <path to romfs folder> --output <path> --extraction trpfs
```

Windows example for trpfs:
```
sv_extractor.exe --romfs <path to romfs folder> --output <path> --extraction trpfs
```

Linux example for trpak:
```
./sv_extractor --trpak <path to trpak> --output <path> --extraction trpak
```

Windows example for trpak:
```
sv_extractor.exe --trpak <path to trpak> --output <path> --extraction trpak
```

Linux example for full:
```
./sv_extractor --romfs <path to romfs folder> --output <path> --extraction full
```

Windows example for full:
```
sv_extractor.exe --romfs <path to romfs folder> --output <path> --extraction full
```

## Extraction options
0. trpfs
1. trpak
2. full