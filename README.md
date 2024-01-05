<img src="https://file.coffee/u/GReDxY1Asexh7lm1HPuv3.png" width="150">


A next-generation, customizable and simple ID system, built in Rust.

---

![Crates.io Version](https://img.shields.io/crates/v/akai)
![GitHub License](https://img.shields.io/github/license/aidakdev/akai)
<a href="https://twitter.com/prfzpx">
    <img src="https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white" />
</a>

### Features
- **Simple.** Akai generates beautiful, minimal IDs that are unique due to time.
- **Customizable.** You can personalize the prefix, the starting timestamp, and you can use a node ID.
- **Robust.** Akai is built in Rust, the world's most loved programming language.
- **Safe.** It is impossible to generate a duplicate ID, due to the uniqueness of generating them based on the timestamp.

### Install

Put the desired version of the crate into the dependencies section of your Cargo.toml:

```toml
[dependencies]
akai = { version = "0.1.0" }
```

### Example
```rs
use akai::{Akai, AkaiPrefix, AkaiOptions};

let prefix = AkaiPrefix {
    content: "user".to_string(),
    allows_lowercase: true
};
    
let akai = Akai::new(AkaiOptions {
    prefix,
    custom_bytes: Some(8),
    custom_timestamp: None
});
    
match akai.generate() {
    Ok(id) => println!("{}", id),
    Err(error) => eprintln!("Error: {}", error)
}
```