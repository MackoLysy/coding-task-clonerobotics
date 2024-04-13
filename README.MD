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


