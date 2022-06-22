# NVIDIA GameStream Presets Tool

![NVIDIA GameStream Presets Tool](.resources/image.png?raw=true "NVIDIA GameStream Presets Tool")

## Building with cargo

```
cargo build --release
```

## Running
Configuration parameters can be passed to NVIDIA GameStream Presets Tool by CLI arguments or loaded from an ini file. CLI arguments have preference over ini ones. By default, a file conf.ini is expected in the working directory.

The assets directory with the default box art png must be placed in the working directory.
### Running with CLI arguments

```
cargo run --release -- -t "C:\Home\Games\Emus\Yuzu\yuzu.exe" -s "C:\\Program Files (x86)\\Steam\\userdata\\99999999\\config\\" -d "C:\\Users\\USER\\AppData\\Local\\NVIDIA Corporation\\Shield Apps\\"
```

### Running with custom ini file

```
cargo run --release -- -c "config\\customconfig.ini"
```

## Portable installation
For running the tool without cargo, place in the same directory:
    - The nvidia-gamestream-presets-tool.exe in target/release directory.
    - The assets directory.
    - The conf.ini.
After properly setting your config in the conf.ini file, just double click in nvidia-gamestream-presets-tool.exe everytime you want to re-create your presets.