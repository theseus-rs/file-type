use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_111: FileFormat = FileFormat {
    id: 111,
    source_type: SourceType::Pronom,
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
