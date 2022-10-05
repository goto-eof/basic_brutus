# Basic Brutus :)

## What is the project Basic Brutus?

Basic Brutus is a HTTP/HTTPS Basic Authentication dictionary attack tool implemented in Rust programming language. The aim of this project is to make some experiments with Rust.

## How to use it?

```
cargo run -q
```

Then use `-u` to specify the usenrame, `-t` to specify the target uri and `-d` to specify the complete path to the password dictionary.

```
-u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```

## Command line:

To view all commands:

```
basic_brutus --help
```

To run a dictionary attack:

```
basic_brutus -u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```
