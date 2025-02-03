use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_933: FileFormat = FileFormat {
    id: 933,
    source_type: SourceType::Pronom,
    name: "Obsidium Project File",
    extensions: &["opf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
