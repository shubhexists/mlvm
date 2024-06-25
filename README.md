# Multi Language Version Manager
`mlvm` is a command line tool that can be used to switch between versions for different languages.
Drop of a ✨ if you are here. It would mean a lot : )


[DEMO.webm](https://github.com/shubhexists/mlvm/assets/110319892/b5a0b3f4-6cb3-4ce4-9635-a31b3f93aa04)

# Features 
1) Fast, written in Rust
2) A single binary to control versions of multiple languages, without increasing the space
3) Secure, as it uses symlinks instead of changing env.
4) Easy to use

# Current Supported Languages
- [x] Node
- [ ] Go

More languages to be added soon!

# Installation
## Using Cargo
You can install mlvm from [Cargo](https://crates.io/crates/mlvm) directly
```
cargo install mlvm
```
## Build From Source
1) Clone this repository by running the command
```
git clone https://github.com/shubhexists/mlvm
```
2) `cd` into the directory and run
```
cargo build --release
```
 This will create binaries for the project.
 
3) Export the path of the executable (It is in the `/target/release/` directory .) For eg,
```
   export PATH="$PATH:/home/jerry/Desktop/mlvm/target/release"
```
4) You are all set to use mlvm :)

# Major Commands
1) To install a new version
```
mlvm language_name install version_number
```
Eg. 
```
mlvm node install 16 // installs node 16
```
2) To list all the versions of a language
```
mlvm language_name ls
```
3) To remove a version of any language
```
mlvm language_name remove version_number
```
4) To switch between different version of a language
```
mlvm language_name use version_number
```
5) To execute a piece of code without actaully changing the version in the shell (Work In Progress)
```
mlvm language_name exec version_number file_to_execute
```
6) To create version aliases for a language (Work In Progress)
```
mlvm language_name alias add version_number alias_name
```

For more details about commands, and flags like `--debug` and `--no-default`, Refer to COMMANDS.md (Work In Progress).

# Thanks
If you read till here, thanks for showing interest in the project :)
