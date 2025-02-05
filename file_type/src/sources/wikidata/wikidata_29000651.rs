use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29000651: FileFormat = FileFormat {
    id: 29_000_651,
    source_type: SourceType::Wikidata,
    name: "WLD",
    extensions: &["wld"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
