use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2113: FileFormat = FileFormat {
    id: 2_113,
    source_type: SourceType::Pronom,
    name: "Calendar Creator Event",
    extensions: &["ce3"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 202,
    }],
};
