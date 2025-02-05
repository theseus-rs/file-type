use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123693494: FileFormat = FileFormat {
    id: 123_693_494,
    source_type: SourceType::Wikidata,
    name: "Module Definition file",
    extensions: &["def"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
