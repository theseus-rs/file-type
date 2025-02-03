use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851201: FileFormat = FileFormat {
    id: 105_851_201,
    source_type: SourceType::Wikidata,
    name: "Windows 98-7 Desktop Theme",
    extensions: &["the", "theme"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
