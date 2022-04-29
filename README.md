Rust Keylogger
==============

This is a keylogger for Linux and Windows(WIP). It supports rootless and rooted linux enviroments. 

* Forked from: https://github.com/gsingh93/keylogger and https://github.com/thomaslienbacher/win-keylogger-rs
* Rootless x11 keylogger developed by Pixelcoda/p0indexter

I am not responsible for how you use this tool.

## Installation

Clone the repository:

```
$ git clone git@github.com:pixelcoda/keylogger.git
$ cd keylogger
```

Build the code:

```$ cargo build --release```

You can run the code with Cargo or directly from the target directory. Note that the keylogger must be run as the root user:

```
$ sudo cargo run --release -- -h
$ sudo ./target/release/keylogger -h
```

You can move the `keylogger` binary wherever you want. For example, you can put it in `/usr/local/bin` or in any other directory in your path.

## Usage

```
$ sudo cargo run -- -h

Usage: target/release/keylogger [options]

Options:
    -h --help           prints this help message
    -v --version        prints the version
    -d --device DEVICE  specify the device file
    -f --file FILE      specify the file to log to
```

If the `-f` flag is not specified, the file `keys.log` is used.

If you would like to run the keylogger in the background, append an `&` to the end of the command. If you would like to run the keylogger as a daemon or at startup, use init script/service manager that comes with your distro. An example `systemd` file is provided.

## License

[MIT](https://github.com/pixelcoda/keylogger/blob/master/LICENSE.txt)
