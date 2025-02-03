use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_124970543: FileFormat = FileFormat {
    id: 124_970_543,
    source_type: SourceType::Wikidata,
    name: "MIX message data file",
    extensions: &["mix"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
