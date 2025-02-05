use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_939: FileFormat = FileFormat {
    id: 939,
    source_type: SourceType::Pronom,
    name: "ScanIt Document",
    extensions: &["sid"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
