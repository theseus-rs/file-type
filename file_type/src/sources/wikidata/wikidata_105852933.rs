use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852933: FileFormat = FileFormat {
    id: 105_852_933,
    source_type: SourceType::Wikidata,
    name: "Crimson Fields level (with rem)",
    extensions: &["src"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
