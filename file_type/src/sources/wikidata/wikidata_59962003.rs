use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_59962003: FileFormat = FileFormat {
    id: 59_962_003,
    source_type: SourceType::Wikidata,
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
