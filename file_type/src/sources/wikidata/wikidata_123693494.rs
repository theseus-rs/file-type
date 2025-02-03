use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123693494: FileFormat = FileFormat {
    id: 123_693_494,
    source_type: SourceType::Wikidata,
    name: "Module Definition file",
    extensions: &["def"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
