use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_938: FileFormat = FileFormat {
    id: 938,
    source_type: SourceType::Pronom,
    name: "Information or Setup File",
    extensions: &["inf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
