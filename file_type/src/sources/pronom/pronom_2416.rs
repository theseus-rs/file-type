use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2416: FileFormat = FileFormat {
    id: 2_416,
    source_type: SourceType::Pronom,
    name: "Visual Basic Binary Form File",
    extensions: &["frx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
