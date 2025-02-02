use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_76515023: FileFormat = FileFormat {
    id: 76_515_023,
    source_type: SourceType::Wikidata,
    name: "Safari Web History",
    extensions: &["webhistory"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
