use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856661: FileFormat = FileFormat {
    id: 105_856_661,
    source_type: SourceType::Wikidata,
    name: "Windows URL shortcut",
    extensions: &["url"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
