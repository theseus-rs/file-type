use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_119: FileFormat = FileFormat {
    id: 119,
    source_type: SourceType::Pronom,
    name: "AutoCAD Plot Configuration File",
    extensions: &["pc2"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
