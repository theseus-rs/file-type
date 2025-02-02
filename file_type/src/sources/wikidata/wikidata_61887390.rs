use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_61887390: FileFormat = FileFormat {
    id: 61_887_390,
    source_type: SourceType::Wikidata,
    name: "EndNote Library format 1-9",
    extensions: &["enl"],
    media_types: &["application/x-endnote-library"],
    internal_signatures: &[],
    related_formats: &[],
};
