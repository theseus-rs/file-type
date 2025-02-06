use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_114: FileFormat = FileFormat {
    id: 114,
    source_type: SourceType::Pronom,
    name: "AutoCAD Template Menu File",
    extensions: &["mnu"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
