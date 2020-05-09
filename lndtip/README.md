# lndtip

## lndtip.toml

```toml
host="127.0.0.1:10009"
cert="""-----BEGIN CERTIFICATE-----
....
-----END CERTIFICATE-----"""
macaroon="ABCD00.."
```

## Inspiration

* https://github.com/robclark56/lightningtip-PHP
* https://github.com/michael1011/lightningtip
* https://github.com/conscott/get-lightning-paid

## TODO

* [x] implement server side events
* [ ] implement lnd_client::IntoConnectionInfo to load config

## Api

* http://127.0.0.1:3030/check_invoice?r_hash=abcdef...
* http://127.0.0.1:3030/generate_invoice?satoshi=1000&description=nothing&expiry=0
* http://127.0.0.1:3030/watchinvoice?r_hash=abcdef...

### Links

* https://github.com/seanmonstar/warp/blob/master/examples/todos.rs

## EventSource 

Polls the lnd server every 2 seconds, maybe rewrite to using grpc stream?
Returns

data:
```json
{"status":"SETTLED","settled":true,"expiry":3600}
```

Check streaming with curl
```shell
curl -N   http://127.0.0.1:3030/watchinvoice?r_hash=abcdef...
```

### Links

* https://github.com/seanmonstar/warp/blob/master/examples/sse.rs
* https://github.com/seanmonstar/warp/blob/master/src/filters/sse.rs
* https://developer.mozilla.org/nl/docs/Web/API/EventSource

## NGINX config

1. Copy the frontend directory to a directory on www server.
2. Run the lndtip in a tmux session on port 3030

```                                                             
location /lndtip {                                                                                       
   rewrite /lndtip/(.*) $1 break;
   // needed for application/json post
   proxy_pass_header Content-Type; 
   // streaming events
   proxy_set_header Connection '';                                              
   proxy_http_version 1.1;                                                      
   chunked_transfer_encoding off;                                               
   proxy_buffering off;                                                         
   proxy_cache off;
   // include querystring
   proxy_pass http://127.0.0.1:3030/$1$is_args$args;                            
}                                                                               
```

## License

Copyright(c) 2020 David Rasch

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License (GPL) as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but without any warranty; without even the implied warranty of merchantability or fitness for a particular purpose. See the GNU General Public License for more details.
