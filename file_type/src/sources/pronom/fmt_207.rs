use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_207: FileFormat = FileFormat {
    id: 933,
    puid: "fmt/207",
    name: "Obsidium Project File",
    extensions: &["opf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
