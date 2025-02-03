use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864118: FileFormat = FileFormat {
    id: 105_864_118,
    source_type: SourceType::Wikidata,
    name: "AUMenu Menu Definition (with rem)",
    extensions: &["mdf"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
