use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130603160: FileFormat = FileFormat {
    id: 130_603_160,
    source_type: SourceType::Wikidata,
    name: "REBOL file format",
    extensions: &["r", "r3", "reb"],
    media_types: &["text/x-rebol"],
    signatures: &[],
    related_formats: &[],
};
