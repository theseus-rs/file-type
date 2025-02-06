use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_88: FileFormat = FileFormat {
    id: 88,
    source_type: SourceType::Pronom,
    name: "AutoCAD Font Mapping Table",
    extensions: &["fmp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
