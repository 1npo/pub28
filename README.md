# pub28
**A street address standardizer compliant with USPS Publication 28. Use as a CLI tool, library, or REST API server.**

Applies all relevant standards defined in [USPS Publication 28](https://pe.usps.com/text/pub28/welcome.htm) to one or more given US street addresses. See the [Documentation](#Documentation) section below for more details.

*Does not provide address validation* or apply any standards from the publication that depend on access to paid USPS data or services (such as the City State file or the ZIP+4 file).

Expects valid and complete street addresses for input (eg, all address fields are populated, and if in string format, are comma-delimited). Tries to fill some gaps and correct or handle some common errors, but performs best when inputs are valid and complete.

# Dependencies
## Required
* `serde` for working with JSON (MIT or Apache-2.0)
* `clap` for processing CLI arguments (MIT or Apache-2.0)
* `config` for managing app settings and configuration file
## Optional
* `arctix-web` for using `pub28` as a REST API
* `polars` for working with Excel files and dataframes

# Install
Run the following Cargo command in your project directory:
```console
cargo add pub28 --features api dataframes
```
Or add the following line to your Cargo.toml:
```console
pub28 = { version = "0.1.0", features = ["api", "dataframes"] }
```

# Usage
```console
$ pub28 --help
...
```

# Documentation
* See the included [docs/pub28.pdf](pub28.pdf) for a PDF version of USPS Publication 28
* See [docs/supported_standards.md](supported_standards.md) for an exhaustive list of all the transformations that this crate will apply to make the given address(es) conform to Publication 28
* See `[TBD]` for documentation on the library API and REST API

# License
`pub28` is dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE)