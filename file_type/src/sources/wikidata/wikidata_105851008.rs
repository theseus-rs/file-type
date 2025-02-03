use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851008: FileFormat = FileFormat {
    id: 105_851_008,
    source_type: SourceType::Wikidata,
    name: "Windows Desktop Theme (with rem)",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
