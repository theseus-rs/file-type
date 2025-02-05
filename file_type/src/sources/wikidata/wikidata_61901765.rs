use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61901765: FileFormat = FileFormat {
    id: 61_901_765,
    source_type: SourceType::Wikidata,
    name: "EndNote Import File",
    extensions: &["enr", "enw"],
    media_types: &["application/x-endnote-refer"],
    signatures: &[],
    related_formats: &[],
};
