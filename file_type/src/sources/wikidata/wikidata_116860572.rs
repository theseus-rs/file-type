use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116860572: FileFormat = FileFormat {
    id: 116_860_572,
    source_type: SourceType::Wikidata,
    name: "Stock Tracker Report File",
    extensions: &["srw"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
