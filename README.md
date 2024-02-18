# Hasher
A GUI tool which holds a catalog of various hash and checksums algorithms.

## Demo
![Gif-Hasher](https://github.com/ByteSudoer/Hasher/assets/88513682/205d25bd-aa51-4cd5-a831-77495a4e65c8)


## Installation
- [From the AUR](#From-the-AUR)
- [From Source Code](#from-Source-Code)
### From the AUR
##### Install an AUR helper like:
- `paru`
- `yay`

and then run this command
```bash 
paru -S hasher
```
### From Source Code

- Check the [egui](https://github.com/emilk/egui/tree/master) official documentation to see what packages need to be installed depending on you distribution.
- Download `rustup` tool via this link [rust-lang](https://www.rust-lang.org/tools/install) or check the proper documentation of your linux distribution.

- clone this repository
```bash
git clone https://github.com/ByteSudoer/Hasher.git
```

- build the project
```bash
cd Hasher
cargo build --release
```
- run the project
```bash 
./target/release/hasher
```


## TODO
- [ ] Add a Dockerfile.
- [ ] Add Support for VNC servers.
- [ ] Add copy button to UNIX clipboard.
- [ ] Add File checksum options.
- [ ] Add the package to the AUR.
