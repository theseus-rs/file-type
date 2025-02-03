use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856700: FileFormat = FileFormat {
    id: 105_856_700,
    source_type: SourceType::Wikidata,
    name: "Qt User Interface",
    extensions: &["ui"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
