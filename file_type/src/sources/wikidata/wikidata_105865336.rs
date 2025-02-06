use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105865336: FileFormat = FileFormat {
    id: 105_865_336,
    source_type: SourceType::Wikidata,
    name: "PC-Axis data (var 1)",
    extensions: &["px"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
