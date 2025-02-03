use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_210: FileFormat = FileFormat {
    id: 210,
    source_type: SourceType::Pronom,
    name: "Desktop Color Separation File",
    extensions: &["dcs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
