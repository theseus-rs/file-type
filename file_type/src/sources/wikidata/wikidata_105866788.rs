use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105866788: FileFormat = FileFormat {
    id: 105_866_788,
    source_type: SourceType::Wikidata,
    name: "PGN (Portable Gaming Notation) Compressed format",
    extensions: &["pgc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
