# HttpBoy

Http client for developers, capable of running requests based on specs written in `TOML` (with templating!).

## Features
- run requests described with `TOML`
- run them in series (depending on each other)
- use templating for rich interaction and value reuse

## Usage

Prepare your RFC-2616 compliant request in a text file. Then, run `httboy request.http` and observe the results in your stdout.
