use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856661: FileFormat = FileFormat {
    id: 105_856_661,
    source_type: SourceType::Wikidata,
    name: "Windows URL shortcut",
    extensions: &["url"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
