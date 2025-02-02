use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_101: FileFormat = FileFormat {
    id: 101,
    source_type: SourceType::Pronom,
    name: "AutoCAD Linetype Definition File",
    extensions: &["lin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
