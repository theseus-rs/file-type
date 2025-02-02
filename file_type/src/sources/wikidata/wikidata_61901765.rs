use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61901765: FileFormat = FileFormat {
    id: 61_901_765,
    source_type: SourceType::Wikidata,
    name: "EndNote Import File",
    extensions: &["enr", "enw"],
    media_types: &["application/x-endnote-refer"],
    internal_signatures: &[],
    related_formats: &[],
};
