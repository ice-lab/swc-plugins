# SWC plugins for ice.js
Plugins of SWC, written in rust.

- keep-export
- keep-platform
- remove-export

## Prepare

1. Make sure cargo installed in your device.

2. Run `cargo install` in the root of project.

3. Add wasm32-wasi as a target

   - 
     ```
     rustup target add wasm32-wasi
     ```

## Build and publish

1. Check to the target directory.
2. Run `npm run prepublishOnly` to build `.wasm` file.
3. Write CHANGELOG and `npm publish`