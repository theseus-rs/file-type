use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1669: FileFormat = FileFormat {
    id: 1_669,
    source_type: SourceType::Pronom,
    name: "STL (Standard Tessellation Language) Binary",
    extensions: &["stl"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
