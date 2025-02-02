use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_113438957: FileFormat = FileFormat {
    id: 113_438_957,
    source_type: SourceType::Wikidata,
    name: "EndNote Library 20",
    extensions: &["enl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
