# lndtip

## lndtip.toml

host="127.0.0.1:10009"
cert="""-----BEGIN CERTIFICATE-----
....
-----END CERTIFICATE-----"""
macaroon="ABCD00.."

## Inspiration

* https://github.com/robclark56/lightningtip-PHP
* https://github.com/michael1011/lightningtip
* https://github.com/conscott/get-lightning-paid

## TODO

* implement lnd_client::IntoConnectionInfo to load config

## Examples

http://127.0.0.1:3030/check_invoice?r_hash=abcdef...
http://127.0.0.1:3030/generate_invoice?satoshi=1000&description=nothing&expiry=0


## License

Copyright(c) 2020 David Rasch

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License (GPL) as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but without any warranty; without even the implied warranty of merchantability or fitness for a particular purpose. See the GNU General Public License for more details.