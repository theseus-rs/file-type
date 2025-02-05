use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_218: FileFormat = FileFormat {
    id: 218,
    source_type: SourceType::Pronom,
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
