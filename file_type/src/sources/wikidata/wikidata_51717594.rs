use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51717594: FileFormat = FileFormat {
    id: 51_717_594,
    source_type: SourceType::Wikidata,
    name: "Pocket Word Template",
    extensions: &["pwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
