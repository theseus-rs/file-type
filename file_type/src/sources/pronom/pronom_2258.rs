use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2258: FileFormat = FileFormat {
    id: 2_258,
    source_type: SourceType::Pronom,
    name: "Apple iWork Numbers",
    extensions: &[],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
