use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_113438957: FileFormat = FileFormat {
    id: 113_438_957,
    source_type: SourceType::Wikidata,
    name: "EndNote Library 20",
    extensions: &["enl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
