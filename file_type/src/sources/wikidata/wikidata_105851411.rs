use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851411: FileFormat = FileFormat {
    id: 105_851_411,
    source_type: SourceType::Wikidata,
    name: "Windows 98-7 Desktop Theme (with rem)",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
