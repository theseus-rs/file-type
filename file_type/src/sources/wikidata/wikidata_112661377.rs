use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112661377: FileFormat = FileFormat {
    id: 112_661_377,
    source_type: SourceType::Wikidata,
    name: "VIZ Material XML Import",
    extensions: &["xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
