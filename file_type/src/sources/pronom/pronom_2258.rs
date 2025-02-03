use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2258: FileFormat = FileFormat {
    id: 2_258,
    source_type: SourceType::Pronom,
    name: "Apple iWork Numbers",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
