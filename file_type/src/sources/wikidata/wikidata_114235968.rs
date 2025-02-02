use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_114235968: FileFormat = FileFormat {
    id: 114_235_968,
    source_type: SourceType::Wikidata,
    name: "SYBYL format",
    extensions: &["sml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
