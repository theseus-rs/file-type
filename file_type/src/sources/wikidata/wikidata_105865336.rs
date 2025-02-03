use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105865336: FileFormat = FileFormat {
    id: 105_865_336,
    source_type: SourceType::Wikidata,
    name: "PC-Axis data (var 1)",
    extensions: &["px"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
