use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123349943: FileFormat = FileFormat {
    id: 123_349_943,
    source_type: SourceType::Wikidata,
    name: "Family Origins Database",
    extensions: &["fow"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
