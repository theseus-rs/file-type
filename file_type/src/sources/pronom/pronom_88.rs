use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_88: FileFormat = FileFormat {
    id: 88,
    source_type: SourceType::Pronom,
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
