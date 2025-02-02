use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123594002: FileFormat = FileFormat {
    id: 123_594_002,
    source_type: SourceType::Wikidata,
    name: "Norton Change Directory Persistent Cache File",
    extensions: &["ncd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
