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
2. Open a terminal where you have the binaries and run the following command:

Linux examples:
```
./sv_extractor --path <path to romfs folder> --output <path> --extraction trpfs
```

```
./sv_extractor --path <path to .trpak file> --output <path> --extraction trpak
```

```
./sv_extractor --path <path to romfs folder> --output <path> --extraction full
```

Windows examples:
```
sv_extractor.exe --path <path to romfs folder> --output <path> --extraction trpfs
```

```
sv_extractor.exe --path <path to .trpak file> --output <path> --extraction trpak
```

```
sv_extractor.exe --path <path to romfs folder> --output <path> --extraction full
```

## Extraction options
0. trpfs
1. trpak
2. full

## Notes
* When extracting trpak's either individually or with the full extraction, the decompressed files will be in the same directory as the compressed file.