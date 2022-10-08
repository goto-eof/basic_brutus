# Basic Brutus :)

## What is the project Basic Brutus?

Basic Brutus is a HTTP/HTTPS Basic Authentication dictionary attack tool implemented in Rust programming language. The aim of this project is to make some experiments with Rust.

## How it works?

Basic Brutus creates a group of threads on which it distributes work while the dictionary file is read line by line. The first thread that manages to get the password X from the channel will be the thread that will also have to process it, which means making an attempt to verify the matching of the username and the password. Basic Brutus allows the use of many usernames that are loaded from a file.

## How to use it?

```
cargo run -q
```

Then use `-u` to specify the usenrame, `-t` to specify the target uri, `-d` to specify the complete path to the password dictionary, `-v` to specify the usernames file.

```
-u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```
or
```
-t https://website.com/something -v /PATH/TO/USERNAMES_FILE.txt  -d /PATH/TO/DICTIONARY.txt
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
./basic_brutus -u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```
or
```
./basic_brutus -v /PATH/TO/USERNAMES_FILE.txt -t https://website.com/something -d /PATH/TO/DICTIONARY.txt
```

## Environment variables

The environment variables are found in the .env file of the project and allows you to alter the behavior of the application.

- `CHANNEL_BUFFER=10000000` - buffer size of the inter-thread communication channel. The default value is 10000000.
- `MAX_NUM_THREADS=12` - if specified, the default thread count (corresponding to the number of processor cores) will be overwritten by the value specified by the user in the .env file.

## Comparison

```
MacOS  -  M1 Pro            -  8 thread   ->  ~42s
Ubuntu -  Ryzen 7 (4800H)   -  16 thread  ->  ~133s
MacOS  -  M1 Pro            -  1 thread   ->  ~240s
```

## Dictionaries

Dictionary files can be found [here](https://github.com/berandal666/Passwords).

## Post Scriptum

If you are using Linux, you should install some packages:

```
sudo apt-get install pkg-config

sudo apt-get install libssl-dev
```

Tested on MacOS and Ubuntu and (today) it works.
