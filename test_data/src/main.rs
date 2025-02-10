//! This crate generates test files based on the source signatures.

#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

use file_type::format::SourceType;
use file_type::sources::{pronom, wikidata};

mod generator;
mod supported_formats;
mod test_signature;

fn main() -> anyhow::Result<()> {
    supported_formats::generate()?;
    generator::generate(&SourceType::Pronom, pronom::FILE_FORMATS)?;
    generator::generate(&SourceType::Wikidata, wikidata::FILE_FORMATS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() -> anyhow::Result<()> {
        main()
    }
}
