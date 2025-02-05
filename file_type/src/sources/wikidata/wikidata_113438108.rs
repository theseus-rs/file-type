use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113438108: FileFormat = FileFormat {
    id: 113_438_108,
    source_type: SourceType::Wikidata,
    name: "EndNote Library X - X9",
    extensions: &["enl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
