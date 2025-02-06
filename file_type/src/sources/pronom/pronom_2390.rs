use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2390: FileFormat = FileFormat {
    id: 2_390,
    source_type: SourceType::Pronom,
    name: "reStructuredText",
    extensions: &["rst"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
