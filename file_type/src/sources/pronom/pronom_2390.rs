use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2390: FileFormat = FileFormat {
    id: 2_390,
    source_type: SourceType::Pronom,
    name: "reStructuredText",
    extensions: &["rst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
