![alt basic brutus](./header2.jpg)

# What is the project «Basic Brutus»?

Basic Brutus is a HTTP/HTTPS Basic Authentication and cross platform dictionary attack tool implemented in Rust programming language. The aim of this project is to make some experiments with threads in Rust.

# How it works?

Basic Brutus creates a group of threads on which it distributes work while the dictionary file is read line by line. The first thread that manages to get the password X from the channel will be the thread that will also have to process it, which means making an attempt to verify the matching of the username and the password. Basic Brutus ca use a username passed as parameter or load usernames from a file.

# For developers

### How to use it?

```
cargo run -q
```

Use `-u` to specify the username, `-t` to specify the target uri, `-d` to specify the complete path to the password dictionary, `-uu` to specify the usernames file, `-v` to specify the verbose mode, `-f` to specify the maximum number of attemps if request fails (default: infinite).

```
-u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt -v true
```

or

```
-t https://website.com/something -uu /PATH/TO/USERNAMES_FILE.txt  -d /PATH/TO/DICTIONARY.txt -v true
```

### Environment variables

The environment variables are found in the .env file of the project and allows you to alter the behavior of the application.

- `CHANNEL_BUFFER=10000000` - buffer size of the inter-thread communication channel. The default value is 10000000.
- `MAX_NUM_THREADS=12` - if specified, the default thread count (corresponding to the number of processor cores) will be overwritten by the value specified by the user in the .env file.

# Command line:

To view all commands:

```
basic_brutus --help
```

To run a dictionary attack:

```
./basic_brutus -u USERNAME -t https://website.com/something -d /PATH/TO/DICTIONARY.txt -v true
```

or

```
./basic_brutus -uu /PATH/TO/USERNAMES_FILE.txt -t https://website.com/something -d /PATH/TO/DICTIONARY.txt -v true
```

# Dictionaries

Dictionary files can be found [here](https://github.com/berandal666/Passwords).

# Tests

Tested on `MacOS`, `Ubuntu` and `Windows 11` and (today) it works.

# Comparison

I used the same dictionary file on three different notebooks with energy saving off.

```
1. MacOS       -  M1 Pro                -  8 threads   ->  43.34s
1. MacOS       -  M1 Pro                -  8 threads   ->  42.08s
1. MacOS       -  M1 Pro                -  8 threads   ->  43.63s
-----------------------------------------------------------------
2. Windows 11  -  Intel i7-10750H       -  12 threads  ->  31.80s
2. Windows 11  -  Intel i7-10750H       -  12 threads  ->  60.15s
2. Windows 11  -  Intel i7-10750H       -  12 threads  ->  30.98s

2. Ubuntu      -  Intel i7-10750H       -  12 threads  ->  48.61s
2. Ubuntu      -  Intel i7-10750H       -  12 threads  ->  61.04s
2. Ubuntu      -  Intel i7-10750H       -  12 threads  ->  65.48s
-----------------------------------------------------------------
3. Ubuntu      -  AMD Ryzen 7 (4800H)   -  16 threads  ->  139.42s
3. Ubuntu      -  AMD Ryzen 7 (4800H)   -  16 threads  ->  140.37s
3. Ubuntu      -  AMD Ryzen 7 (4800H)   -  16 threads  ->  136.08s

```

# Linux users

If you are using Linux, you should install some packages:

```
sudo apt-get install pkg-config

sudo apt-get install libssl-dev
```

# Mac OS users

if you downloaded the executable, make sure you allow execution of not verified application. So that go to Settings > Security & Privacy > General, Click on "Allow Anyway", try to execute application again and click on Open option.

# Download

[Here](https://github.com/goto-eof/basic_brutus/releases) you can find the executables.

if any problems arise, feel free to [contact me](http://andre-i.eu/#contactme).
