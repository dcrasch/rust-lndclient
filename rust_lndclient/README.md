# Rust Lightning Network Client


## Other projects

* https://github.com/LightningPeach/lnd-rust

## Protos

* https://raw.githubusercontent.com/lightningnetwork/lnd/master/lnrpc/rpc.proto

## lndtip.toml

host="127.0.0.1:10009"
cert="""-----BEGIN CERTIFICATE-----
....
-----END CERTIFICATE-----"""
macaroon="ABCD00.."

## Run program

cargo run

## Hacks for grpc under windows

httpbis-0.8.0\src\socket.rs:103:10
```
Pin<Box<dyn Future<Output = io::Result<Pin<Box<dyn StreamItem + Send>>>> + Send>> 
```

\grpc\src\client_stub.rs:58:15
```
#[cfg(not(unix))]
    fn new_plain_unix(addr: &str, conf: ClientConf) -> grpc_Result<Self> {
        unimplemented!("new plain unix not implemented");      
    }
```

Send patch or use https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section

## License

Rust lightningnetwork client

Copyright(c) 2020 David Rasch

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License (GPL) as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but without any warranty; without even the implied warranty of merchantability or fitness for a particular purpose. See the GNU General Public License for more details.
