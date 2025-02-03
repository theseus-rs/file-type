use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_147: FileFormat = FileFormat {
    id: 147,
    source_type: SourceType::Pronom,
    name: "Schedule+ Contacts",
    extensions: &["scd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
