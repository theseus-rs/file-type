use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851201: FileFormat = FileFormat {
    id: 105_851_201,
    source_type: SourceType::Wikidata,
    name: "Windows 98-7 Desktop Theme",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
