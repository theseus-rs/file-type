use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61901680: FileFormat = FileFormat {
    id: 61_901_680,
    source_type: SourceType::Wikidata,
    name: "EndNote Connection File",
    extensions: &["enz"],
    media_types: &["application/x-endnote-connection"],
    internal_signatures: &[],
    related_formats: &[],
};
