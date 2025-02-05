use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_109: FileFormat = FileFormat {
    id: 109,
    source_type: SourceType::Pronom,
    name: "AutoCAD Compiled Menu",
    extensions: &["mnc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
