# PRONOM 

This utility exports the PRONOM data from https://www.nationalarchives.gov.uk/PRONOM to the data directory in the
`file_type` crate.  Before running this utility, the maximum PUID values should be updated in the PUID_TYPES so that the
new definitions are included in the export.

## Usage

To export the PRONOM data, run the following command:

```sh
cargo run --bin pronom
```
