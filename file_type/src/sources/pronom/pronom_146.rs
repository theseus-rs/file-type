use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_146: FileFormat = FileFormat {
    id: 146,
    source_type: SourceType::Pronom,
    name: "AutoCAD ACIS Export File",
    extensions: &["sat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
