use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_8062943: FileFormat = FileFormat {
    id: 8_062_943,
    source_type: SourceType::Wikidata,
    name: "ZAP File",
    extensions: &["zap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
