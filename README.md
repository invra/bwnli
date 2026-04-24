# Bitwig Studio Novation Launchpad Info

Tool which displays Bitwig Studio incoming MIDI information so you can use
that information on an OBS Scene as an example. 

## Support

Tested on:
- Linux (NixOS)
- macOS (26 Tahoe using nix-darwin)

Supports Bitwig 6.

## What is the behind the scenes and what's needed?

We in this project have 3 things:

- Bitwig extension to get data from
- Server to receive data and host;
- Website which shows that information

This won't get sent to external servers-
it will only get sent to the one you specify & need to self-host.

## Structure

Information on separation of handling done by each side of this project
and the outlines of how complex one thing is able to be. This is sorted
so tasks that are important to happen in Bitwig are the only operations
which happen in Bitwig, this keeps the app, and audio engine stable(er).

### Kotlin Handling
The Kotlin side is intentionally kept minimal, this is to keep Bitwig's
process as stable as possible, less susceptible to us crashing the app,
or audio server. 
If the Rust server crashes, Bitwig is unaffected.

The extension will only:

- Reads incoming controller data from Bitwig
- Forwards it to the Rust server over a WebSocket connection

### Rust Handling
The Rust side handles everything else:

- **Receiving MIDI data** — the Bitwig extension sends data here; 
  the server parses it into native Rust types
- **Hosting the web UI** — a local website that displays live device state
- **WebSocket broadcasting** — pushes state updates to connected clients automatically

## Building the project(s)

You have three (3) modules so we'll separate it out into multi-sections

### Bitwig sender extension

#### Dependencies

- Kotlin (2.2.xx)
- Java (17)
- Gradle (not provided in the project because contributing is suggested through nix)

#### Build command

```
  gradle build
```

This will actually build the Gradle project, and also install it in
`Documents/Bitwig Studio/Extensions/<extension-name>.bwextension`

### Website

Currently with the extensive work to the server, it's not supported yet the way we want where
the server will actually build the website- until then this is what you have to do

#### Dependencies

- rustup (mode=complete and `wasm32-unknown-unknown`)
- trunk

#### Build command

```
  trunk build --release
```

#### Execution command

```
  trunk serve --release
```

### Dispatch Server

Requires the same Rust toolchain as the Website,
just without trunk.

#### Dependencies

Uses some of dependencies from Website, so install them

#### Build command

```
  cargo build --release
```

#### Execution command

```
  cargo run --release
```

## License

All current code is licensed under Apache-2.0.
