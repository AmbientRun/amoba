## AMOBA

AMOBA is a moddable MOBA game written with the Ambient engine.

## Usage

You should install Ambient first following the instruction [here](https://ambientrun.github.io/Ambient/user/installing.html).

## Modding

Use `ambient run` to run for the first time.

You can try to comment out some `bin` in `Cargo.toml` to see how each mod is added.

For example, if you comment out the `map` mod:

```toml
# [[bin]]
# name = "client_map"
# path = "src/map/client.rs"
# required-features = ["client"]
```

Then run with `ambient run --clean-build`. You will see the game map is gone.
