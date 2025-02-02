use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_109: FileFormat = FileFormat {
    id: 109,
    source_type: SourceType::Pronom,
    name: "AutoCAD Compiled Menu",
    extensions: &["mnc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
