# Falcon BMS Callbacker

> Invoke BMS callbacks from any networked device!

Falcon BMS Callbacker allows you to invoke BMS callbacks by simply sending them as a UDP datagram.

Givent that BMS runs on the host with the IP address `192.168.1.212`, you can now invoke a callback like this:

```
echo 'SimStepMasterArm' | nc -u 192.168.1.212 2727
```

## What about when I change the key bindings?

`falcon-bms-callbacker` will watch your keyfile for changes and reload it automatically.

## What if I want to listen on a different port?

You can change that in the config file `config.toml`:

```
listen_port = "0.0.0.0"
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