# Basic Brutus :)

## What is the project Basic Brutus?

Basic Brutus is a HTTP/HTTPS dictionary attack tool implemented in Rust programming language. The aim of this project is to make some experiments with Rust.

## How to use it?

```
cargo run -q
```

The use `-u` to specify the usenrame, `-t` to specify the target uri and `-d` to specify the complete path to the dictionary.

```
-u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```
