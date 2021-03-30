# Rust-Lang Web Assembly Project

### Why?
- Rust can help bring lower-level memory management and speed up code in browser
- Browsers are the new "virtual machines", and helps reduce platform dependent problems

### Creating a regex parser for markdown
Experiments here:
https://regex101.com/r/MUn8Fw/1

### Initial Setup
[Most instructions are referenced the wasm documentation](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html).

After creating a project folder; I made use of the pre-built wasm-template as follows
```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
```
Navigate into the new folder, and run the following command, `$ npm init wasm-app www`. This is an abstraction which relies on create-wasm-app

#### To build the app

Run `$ wasm-pack build` to build the project. A `/pkg` folder should appear with the wasm binaries

Open up`www/package.json` and next to "devDependencies", add the "dependencies" field, including a `"<name of package>": "file:../pkg"` entry. Change the imports in the `index.js` accordingly to match \<name of package>. 

I faced an error regarding the `console_error_panic_hook` crate, and removed those codes accordingly. However, it dosen't seem to be a particularly reproducible issue. This crate is very useful - helps to provide clearer error messages.

### Lack of wasm support for different data types eg. `Vec<String>`
[Documented in an Open Github Issue:](https://github.com/rustwasm/wasm-bindgen/issues/111)

For my project, I initially needed to send a `Vec<String>` from Rust Wasm to Javascript. However, I faced `error E0277 (trait bound Box<String>: IntoWasmAbi is not satisfied)` when compiling the program.

[According to the documentation](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/convert/trait.IntoWasmAbi.html), and looking under the foreign types implementation, `Vec<String>` are not yet supported.

A [possible workaround is to use `serde::from`](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/struct.JsValue.html#method.from_serde) which helps to "process (serialize)" a wider range of datatypes to a JSON string. Be sure to check out `serialize` implementations ("impl") first before proceeding to ensure that it accepts your data structure/type. Change to another intermediate data-type if necessary. 

However, it requires activating the `serde-serialize` feature of the wasm-bindgen crate. This can be done in the `cargo.toml` file as shown:
```toml
....

[dependencies]
wasm-bindgen = { version="0.2.63", features=["serde-serialize"]}

....
```
But remember, this now produces a different return type in Rust. Thus, we change the Return Type to JsValue.

```rust
let v: Vec<String> = total.into_iter().collect();
JsValue::from_serde(&v).unwrap()
```
[Click for more examples](https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html).

### How do you know if a crate you choose to use supports WebAssembly?
Backstory: Initially, I ran `wasm-pack build` which runs a few commands. When I faced error, the compiler suggested a few commands that could have gone wrong. 

Run this in the main folder `$ cargo test --target wasm32-unknown-unknown` or `$ cargo build --target wasm32-unknown-unknown` while building the wasm application to check if its dependcies can be built for wasm.
If it fails with errors, the crate likely cannot be used for webassembly. Oh no! How do we know which dependency in the crates installed is causing the problem?

#### Detecting which depencies are unsupported by WebAssembly.
First, we head back to the error.
![f7c0faaddfa1fd933692e150004bc55f.png](:/0f92fa0a54cd425092cf9f25eba739a6)

Copy the file path, and open it up. I decided to open up the `cargo.toml` file in the crate's directory, and identify its name. In this case, the **`fs2` crate seems to be causing the WASM incompatability issues.**

Back in my project directory, I tried using the `cargo tree -p <package name>` command to showcase a list of all the dependencies it uses. With a little guess and check on the various crates, it appears that the  `rust-bert` is not compatible with webassembly, as it relies on `fs2`.

![Screenshot from 2021-03-30 15-08-28.png](:/cf3939f6f0d24ee1b38fc2ee0f3a5079)

> FYI: I wanted to use rust-bert for Natural Language purposes on rust. A check on github shows no support yet for wasm.

[For more info on which rust crates tend to work with Wasm, click on the link to the docs.](https://rustwasm.github.io/docs/book/reference/which-crates-work-with-wasm.html) 

### [Different Rust Wasm Compilation targets](https://rustwasm.github.io/docs/wasm-bindgen/reference/rust-targets.html)

By default, running `wasm-pack build` (wasm-pack is the preferred "official" crate for wasm applications) will set a build target. You can check if it is set up via `$ cargo check --target <target>`. However, you can also set your own rust wasm target with 
`$ rustup target add <target>` Usual target is `wasm32-unknown-unknown`.

### References
https://rustwasm.github.io/docs/book/