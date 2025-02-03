use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_123420503: FileFormat = FileFormat {
    id: 123_420_503,
    source_type: SourceType::Wikidata,
    name: "DropBox file",
    extensions: &["dbox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
