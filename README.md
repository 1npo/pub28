# pub28

A library, CLI tool, and REST API for standardizing street addresses.

Applies all relevant addressing standards defined in [USPS Publication 28](https://pe.usps.com/text/pub28/welcome.htm) to one or more given US street addresses. See the included [docs/pub28.pdf](pub28.pdf) for a PDF version of the publication. See [docs/supported_standards.md](supported_standards.md) for an exhaustive list of all the standards that this crate will attempt to apply to the given address(es).

Does not provide address validation or apply any standards that depend on access to paid USPS data or services (such as the City State file or the ZIP+4 file).

Expects valid and complete street addresses for input. Performs some checks to correct some addressing errors, but performs best with valid and complete inputs that need to be standardized.

Accepts and returns street addresses in multiple ways and formats:

  * CLI arguments
  * Standard input (eg piped from another command on the command-line)
  * JSON objects
  * Excel or CSV files
  * REST API requests/responses
  * Library functions
  * Dataframes

# Install

Run the following Cargo command in your project directory:

```console
cargo add pub28
```

Or add the following line to your Cargo.toml:

```console
pub28 = "0.1.0"
```