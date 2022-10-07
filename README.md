# Basic Brutus :)

## What is the project Basic Brutus?

Basic Brutus is a HTTP/HTTPS Basic Authentication dictionary attack tool implemented in Rust programming language. The aim of this project is to make some experiments with Rust.

## How it works?

Basic Brutus creates a group of threads on which it distributes work as the dictionary file is read line by line. The first thread that manages to get the password X from the channel will be the thread that will also have to process it, which means making an attempt to verify the matching of the username and the password.

## How to use it?

```
cargo run -q
```

Then use `-u` to specify the usenrame, `-t` to specify the target uri and `-d` to specify the complete path to the password dictionary.

```
-u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```

## Command line:

Make executable:

```
cargo build
```

To view all commands:

```
basic_brutus --help
```

To run a dictionary attack:

```
basic_brutus -u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```

## Some statistics

```
MacOS  -  M1 Pro            -  8 thread   ->  ~42s
Ubuntu -  Ryzen 7 (4800H)   -  16 thread  ->  ~140s
MacOS  -  M1 Pro            -  1 thread   ->  ~240s
```

## Post Scriptum

If you are using Linux, you should install some packages:

```
sudo apt-get install pkg-config

sudo apt-get install libssl-dev
```

Tested on MacOS and Ubuntu and (today) it works.


