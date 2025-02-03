use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2710: FileFormat = FileFormat {
    id: 2_710,
    source_type: SourceType::Pronom,
    name: "Enhanced Image Package",
    extensions: &["eip"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
