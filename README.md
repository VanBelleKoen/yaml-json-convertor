# JSON-YAML Converter

A simple CLI tool written in Rust to convert files between JSON and YAML formats.

## Features
- Convert JSON files to YAML.
- Convert YAML files to JSON.
- Automatically detects the file type based on the file extension.

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd JSONYAML_convertor
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. Move the binary to a directory in your PATH:
   ```bash
   mv ./target/release/jsonyaml_convertor /usr/local/bin/jsonyaml
   ```

## Usage

Run the following command to convert a file:
```bash
jsonyaml <file_path>
```

### Example

Convert a JSON file to YAML:
```bash
jsonyaml example.json
```

Convert a YAML file to JSON:
```bash
jsonyaml example.yaml
```

## Requirements
- Rust (for building the project)

## License
This project is licensed under the MIT License.
