use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_82: FileFormat = FileFormat {
    id: 82,
    source_type: SourceType::Pronom,
    name: "AutoCAD Drawing Standards File",
    extensions: &["dws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
