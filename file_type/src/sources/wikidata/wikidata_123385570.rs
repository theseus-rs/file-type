use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123385570: FileFormat = FileFormat {
    id: 123_385_570,
    source_type: SourceType::Wikidata,
    name: "Scene library file",
    extensions: &["scl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
