use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_111: FileFormat = FileFormat {
    id: 111,
    source_type: SourceType::Pronom,
    name: "AutoCAD Menu Resource File",
    extensions: &["mnr", "mnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
