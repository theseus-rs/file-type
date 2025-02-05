use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_938: FileFormat = FileFormat {
    id: 938,
    source_type: SourceType::Pronom,
    name: "Information or Setup File",
    extensions: &["inf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
