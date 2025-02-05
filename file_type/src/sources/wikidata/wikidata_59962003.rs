use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_59962003: FileFormat = FileFormat {
    id: 59_962_003,
    source_type: SourceType::Wikidata,
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
