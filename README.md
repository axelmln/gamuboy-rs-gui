# gamuboy-rs-gui
A GameBoy emulator using rust-sdl2  

It uses [gamuboy-rs](https://github.com/axelmln/gamuboy-rs) as the emulation core.

## Usage

Run the executable with the first argument being the path to the rom you want to play.  
By default the boot sequence is skipped. You can specify a bootrom with the `--bootrom` parameter.  

```
$ ./gamuboy /path/to/rom [--bootrom /path/to/bootrom]
```  

You can also directly run it from source:  
```
$ cd gamuboy-rs-gui
$ cargo run /path/to/rom [--bootrom /path/to/bootrom]
```  

Or you can build it from source:  
```
$ cd gamuboy-rs-gui
$ cargo build --release
```  

To build from source on Linux you may need to first install some dependencies:
```
$ sudo apt-get update
$ sudo apt-get install -y libpulse-dev libasound2-dev
$ sudo apt install -y \
            libsdl2-dev \
            libsdl2-mixer-dev \
            libx11-dev \
            libxext-dev \
            libxrandr-dev \
            libwayland-dev \
            libdrm-dev \
            libgl1-mesa-dev \
            libudev-dev
```  

On Windows you need to set up vckpg:
```
$ cd gamuboy-rs-gui
$ cargo install cargo-vcpkg
$ cargo vcpkg build
```

## Key bindings

| Game Boy        | Keyboard        | Gamepad                          |
|-----------------|-----------------|----------------------------------|
| A               | `A`             | `A`                              |
| B               | `Z`             | `B`                              |
| Start           | `Enter`         | `Start`                          |
| Select          | `Tab`           | `Seletc`                         |
| D-Pad Up        | `Arrow Up`      | `D-Pad Up / Left stick Up`       |
| D-Pad Down      | `Arrow Down`    | `D-Pad Down / Left stick Down`   |
| D-Pad Left      | `Arrow Left`    | `D-Pad Left / Left stick Left`   |
| D-Pad Right     | `Arrow Right`   | `D-Pad Right / Left stick Right` |
