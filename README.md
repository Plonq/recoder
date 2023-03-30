# ReCoder

A simple web-based tool for text encoding, decoding, and hashing. 100% client-side,
100% Rust, 0% cruft.

# Demo

https://recoder.plonq.org

## Why?

Sometimes I need to encode or decode text as a one-off. There are
many existing web tools, but most of them are cluttered,
single-purpose, have ads, etc. Plus, I can't be sure of the privacy.
Creating my own tool solves all these problems.

## Tech Stack

- [Rust](https://www.rust-lang.org)
- [Yew](https://yew.rs) - frontend framework

## Development

If you don't have [Trunk](https://trunkrs.dev) installed, install with:

```bash
cargo install trunk
```

Use Trunk to start a local dev server:

```bash
trunk serve
```

### Deployment

Use Trunk to build for production:

```bash
trunk build --release
```

Output will be in the `dist` directory, which can be statically hosted
anywhere.
