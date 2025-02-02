use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_51717594: FileFormat = FileFormat {
    id: 51_717_594,
    source_type: SourceType::Wikidata,
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
