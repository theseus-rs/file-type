use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123385570: FileFormat = FileFormat {
    id: 123_385_570,
    source_type: SourceType::Wikidata,
    name: "Scene library file",
    extensions: &["scl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
