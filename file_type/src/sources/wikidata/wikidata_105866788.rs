use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105866788: FileFormat = FileFormat {
    id: 105_866_788,
    source_type: SourceType::Wikidata,
    name: "PGN (Portable Gaming Notation) Compressed format",
    extensions: &["pgc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
