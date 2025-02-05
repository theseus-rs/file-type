use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28445584: FileFormat = FileFormat {
    id: 28_445_584,
    source_type: SourceType::Wikidata,
    name: "Application Label Data",
    extensions: &["axc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
