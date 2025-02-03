use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_82: FileFormat = FileFormat {
    id: 82,
    source_type: SourceType::Pronom,
    name: "AutoCAD Drawing Standards File",
    extensions: &["dws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
