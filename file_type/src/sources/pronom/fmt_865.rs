use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_865: FileFormat = FileFormat {
    id: 1_669,
    puid: "fmt/865",
    name: "STL (Standard Tessellation Language) Binary",
    extensions: &["stl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
