# zsh-hist-to-fish

A simple command-line tool to convert Zsh history to Fish shell history format.

This is a port of the python tool [https://github.com/rsalmei/zsh-history-to-fish](https://github.com/rsalmei/zsh-history-to-fish) since I didn't want to install python packages to run this.

## Features

- Converts Zsh history file to Fish history format
- Supports naive conversion of some Zsh-specific syntax to Fish syntax
- Dry-run mode to preview changes without writing to file
- Option to skip syntax conversion

## Installation

### From Releases

1. Go to the [Releases](https://github.com/iamd3vil/zsh-hist-to-fish/releases) page.
2. Download the latest archive for your operating system.
3. Extract the archive.
4. Make the binary executable:
   ```
   chmod +x zsh-hist-to-fish
   ```
5. Move the binary to a directory in your PATH, for example:
   ```
   sudo mv zsh-hist-to-fish /usr/local/bin/
   ```

### From Source

If you have Rust installed, you can build and install from source:

```
cargo install --git https://github.com/iamd3vil/zsh-hist-to-fish.git
```

## Usage

Basic usage:

```
zsh-hist-to-fish
```

This will read from the default Zsh history file (`~/.zsh_history`) and write to the default Fish history file (`~/.local/share/fish/fish_history`).

Options:

- `-o, --output <FILE>`: Specify a custom output file
- `-d, --dry-run`: Run without writing to file (preview mode)
- `-n, --no-convert`: Skip naive syntax conversion
- `-h, --help`: Show help message

Example with options:

```
zsh-hist-to-fish --dry-run --no-convert -o custom_fish_history
```

## How It Works

1. Reads the Zsh history file
2. Optionally performs naive syntax conversion (e.g., `&&` to `; and`)
3. Writes the converted history to the Fish history file format

## Limitations

- The naive syntax conversion is basic and may not cover all Zsh-specific features
- Command timestamps are preserved, but other metadata might be lost in conversion

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
