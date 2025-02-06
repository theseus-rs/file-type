use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_854: FileFormat = FileFormat {
    id: 854,
    source_type: SourceType::Pronom,
    name: "AutoCAD Database File Locking Information",
    extensions: &["dwl"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
