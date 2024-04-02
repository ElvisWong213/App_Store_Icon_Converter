# Icon-image

## About this project

Resize app icon to fit the App Store requirements

## Framework Used

- [clap](https://github.com/clap-rs/clap)
- [image](https://github.com/image-rs/image)
- [zip](https://github.com/zip-rs/zip)

## Requirements

- Rust 1.76.0+

## Setup

Clone repository

```
git clone https://github.com/ElvisWong213/icon-image
```
Build and install binary 

```
cargo install --path .
```

## Usage

```
Usage: icon_image <INPUT_PATH> <OUTPUT_PATH>

Arguments:
  <INPUT_PATH>   Image path. Example: yourIconFolder/yourImage.png
  <OUTPUT_PATH>  Export images to output path. It will create a folder 'output' under your o
utput path. Example: yourProject/Icon

Options:
  -h, --help     Print help
  -V, --version  Print version
```


