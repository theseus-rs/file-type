use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967380: FileFormat = FileFormat {
    id: 27_967_380,
    source_type: SourceType::Wikidata,
    name: "EVT",
    extensions: &["evt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
