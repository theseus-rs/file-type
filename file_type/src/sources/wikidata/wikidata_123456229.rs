use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123456229: FileFormat = FileFormat {
    id: 123_456_229,
    source_type: SourceType::Wikidata,
    name: "CFW Form file",
    extensions: &["cfw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
