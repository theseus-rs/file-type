use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114235968: FileFormat = FileFormat {
    id: 114_235_968,
    source_type: SourceType::Wikidata,
    name: "SYBYL format",
    extensions: &["sml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
