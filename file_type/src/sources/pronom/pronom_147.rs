use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_147: FileFormat = FileFormat {
    id: 147,
    source_type: SourceType::Pronom,
    name: "Schedule+ Contacts",
    extensions: &["scd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
