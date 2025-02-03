use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113438108: FileFormat = FileFormat {
    id: 113_438_108,
    source_type: SourceType::Wikidata,
    name: "EndNote Library X - X9",
    extensions: &["enl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
