# Falcon BMS Callbacker

> Invoke BMS callbacks from any networked device!

Falcon BMS Callbacker allows you to invoke BMS callbacks by simply sending them as a UDP datagram.

Given that BMS _and this program_ both run on the host with the IP address `192.168.1.212`, you can now invoke a callback like this:

```
echo 'SimStepMasterArm' | nc -u 192.168.1.212 9027
```

This should be fairly simple to implement in any language from any networked device you might use in a cockpit build like an arduino or raspberry pi.

Hell, we could even build a customizable BMS button box mobile app with it.

## How do I run this?

- Download the latest release from here: https://github.com/kungfoo/falcon-bms-callbacker/releases
- Unzip it anywhere on your machine.
- Run it. Windows will probably ask for your permission to connect to the network.

## What about when I change the key bindings?

`falcon-bms-callbacker` will watch your keyfile for changes and reload it automatically (it might take a few seconds to detect).

## What if I want to listen on a different port?

You can change that in the config file `config.toml`:

```
listen_port = "9032"
```

## What if I want to bind to a different address?

This will likely only matter to you if you have multiple network interfaces.

You can change that in the config file `config.toml`:

```
listen_address = "192.168.20.35"
```

## I need more logs to figure out what's going on...

You can change that in the config file `config.toml`:

```
# any of: info, debug, trace
log_level = "debug"
```

## Building it

This will build on windows only (BMS is a windows application after all).
It will also build on linux if you cross compile for `ix86_64-pc-windows-gnu`.

Build using:

```
cargo build --release
# or
cargo build --release --target x86_64-pc-windows-gnu
```

On windows the best way to get all the tools needed to build is probably to install MSYS2 and then
the MSYS2 x64 shell and install with `pacaman`:

- base-devel
- cargo
- clang

Clang may not be necessary, depending on what is already installed on the machine, afaict.

