# [Coding Task] Software
Engineer (Rust / C++)

## Before started

Make sure that you have socat installed

```bash
sudo apt-get install socat
```


## To start the test with soct

1. Run script
```bash
sudo scripts/create_socat.sh
```
To create device 

2. Run scrip 
```bash
sudo scripts/permision.sh
```

To add permission for /tmp/ttyV0

next you can start

```bash
export RUST_LOG=info
cargo run -- --serial-port /tmp/ttyV0
```

You can set to trace if you want more logs.
```bash
export RUST_LOG=trace
```

## Test 

After proper build and setting script you can run

```bash
 echo '$0' > /tmp/ttyV1
```
and in diffrent console

```bash
 cat /tmp/ttyV1
```

To test server you can call
```bash
curl http://localhost:7100/start
```