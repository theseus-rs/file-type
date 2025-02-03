use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29650547: FileFormat = FileFormat {
    id: 29_650_547,
    source_type: SourceType::Wikidata,
    name: "Package",
    extensions: &["pack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
