use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856700: FileFormat = FileFormat {
    id: 105_856_700,
    source_type: SourceType::Wikidata,
    name: "Qt User Interface",
    extensions: &["ui"],
    media_types: &["text/xml"],
    signatures: &[],
    related_formats: &[],
};
