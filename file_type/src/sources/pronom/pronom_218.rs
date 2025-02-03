use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_218: FileFormat = FileFormat {
    id: 218,
    source_type: SourceType::Pronom,
    name: "AutoCAD Film Roll",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
