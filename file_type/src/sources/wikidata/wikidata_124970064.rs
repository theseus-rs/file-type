use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124970064: FileFormat = FileFormat {
    id: 124_970_064,
    source_type: SourceType::Wikidata,
    name: "MIX index file",
    extensions: &["mixindex"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
