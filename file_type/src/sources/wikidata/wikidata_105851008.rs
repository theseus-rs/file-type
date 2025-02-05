use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851008: FileFormat = FileFormat {
    id: 105_851_008,
    source_type: SourceType::Wikidata,
    name: "Windows Desktop Theme (with rem)",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
