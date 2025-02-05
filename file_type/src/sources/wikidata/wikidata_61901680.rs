use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61901680: FileFormat = FileFormat {
    id: 61_901_680,
    source_type: SourceType::Wikidata,
    name: "EndNote Connection File",
    extensions: &["enz"],
    media_types: &["application/x-endnote-connection"],
    signatures: &[],
    related_formats: &[],
};
