use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_114235996: FileFormat = FileFormat {
    id: 114_235_996,
    source_type: SourceType::Wikidata,
    name: "SYBYL2 format",
    extensions: &["ml2", "sm2"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
