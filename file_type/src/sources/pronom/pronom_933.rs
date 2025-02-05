use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_933: FileFormat = FileFormat {
    id: 933,
    source_type: SourceType::Pronom,
    name: "Obsidium Project File",
    extensions: &["opf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
