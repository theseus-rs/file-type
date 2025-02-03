use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_114: FileFormat = FileFormat {
    id: 114,
    source_type: SourceType::Pronom,
    name: "AutoCAD Template Menu File",
    extensions: &["mnu"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
