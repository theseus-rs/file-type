use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_101: FileFormat = FileFormat {
    id: 101,
    source_type: SourceType::Pronom,
    name: "AutoCAD Linetype Definition File",
    extensions: &["lin"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
