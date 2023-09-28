Returns cbor diagnostic notation from a given cbor in hex string format.

This module is built for node.js and the browser using webassembly and the rust implementation [cbor-diag](https://github.com/cabo/cbor-diag).

## Installation

```
npm i cbor-diag-wasm --save
```

## Usage

```js
import init, { parse_hex } from 'cbor-diag-wasm';

// this must be called in browser envs
await init();

parse_hex('6568656c6c6f');
```

## License

MIT 2023 - José F. Romaniello
