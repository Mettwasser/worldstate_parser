# Worldstate Parser

A worldstate parser for http://api.warframe.com/cdn/worldState.php - in Rust!

## Not a lib?

~~No, not yet. Why? To make development a bit easier/more comfortable.~~

I just recently transformed it into a lib. If you want to give it a shot, add the git dependency.

## Example

As there's not much documentation right now, you can check [the example](./examples/showcase/), which generates [this file](./showcase/worldstate_parsed.json).

**NOTE:** If you want to run the example yourself, and not fetch the data yourself, run the python script to fetch all the data:

```sh
python init.py
```

after that simply run:

```
cargo run --manifest-path examples/showcase/Cargo.toml
```

## Translation Data

Provided by the awesome [warframe-worldstate-data](https://github.com/WFCD/warframe-worldstate-data) project.

## Internationalization (i18n)

Not yet supported. Not planning to do it any time soon, as I feel like it's not needed as much.
