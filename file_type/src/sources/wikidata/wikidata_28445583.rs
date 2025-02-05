use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445583: FileFormat = FileFormat {
    id: 28_445_583,
    source_type: SourceType::Wikidata,
    name: "Application Label Cache",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
