use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000568: FileFormat = FileFormat {
    id: 29_000_568,
    source_type: SourceType::Wikidata,
    name: "Tacx Fortius",
    extensions: &["caf", "imf", "mrlv", "pgmf", "rlv"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
