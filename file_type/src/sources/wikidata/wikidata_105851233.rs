use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851233: FileFormat = FileFormat {
    id: 105_851_233,
    source_type: SourceType::Wikidata,
    name: "Windows 8-10 Desktop Theme (with rem)",
    extensions: &["theme"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
