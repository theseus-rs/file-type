use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_210: FileFormat = FileFormat {
    id: 210,
    source_type: SourceType::Pronom,
    name: "Desktop Color Separation File",
    extensions: &["dcs"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
