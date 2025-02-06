use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851435: FileFormat = FileFormat {
    id: 105_851_435,
    source_type: SourceType::Wikidata,
    name: "Windows 8-10 Desktop Theme",
    extensions: &["theme"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
