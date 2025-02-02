use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_939: FileFormat = FileFormat {
    id: 939,
    source_type: SourceType::Pronom,
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
