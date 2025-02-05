use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123436321: FileFormat = FileFormat {
    id: 123_436_321,
    source_type: SourceType::Wikidata,
    name: "DARC-F1 Query file",
    extensions: &["f1q"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
