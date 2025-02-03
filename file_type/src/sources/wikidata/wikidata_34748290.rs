use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34748290: FileFormat = FileFormat {
    id: 34_748_290,
    source_type: SourceType::Wikidata,
    name: "T64",
    extensions: &["t64"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
