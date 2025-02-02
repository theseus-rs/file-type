use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130603160: FileFormat = FileFormat {
    id: 130_603_160,
    source_type: SourceType::Wikidata,
    name: "REBOL file format",
    extensions: &["r", "r3", "reb"],
    media_types: &["text/x-rebol"],
    internal_signatures: &[],
    related_formats: &[],
};
