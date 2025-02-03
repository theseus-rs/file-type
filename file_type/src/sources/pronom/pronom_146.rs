use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_146: FileFormat = FileFormat {
    id: 146,
    source_type: SourceType::Pronom,
    name: "AutoCAD ACIS Export File",
    extensions: &["sat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
