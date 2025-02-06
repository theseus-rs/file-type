use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34748290: FileFormat = FileFormat {
    id: 34_748_290,
    source_type: SourceType::Wikidata,
    name: "T64",
    extensions: &["t64"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
