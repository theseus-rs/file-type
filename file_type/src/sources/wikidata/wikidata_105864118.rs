use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864118: FileFormat = FileFormat {
    id: 105_864_118,
    source_type: SourceType::Wikidata,
    name: "AUMenu Menu Definition (with rem)",
    extensions: &["mdf"],
    media_types: &["text/plain"],
    signatures: &[],
    related_formats: &[],
};
