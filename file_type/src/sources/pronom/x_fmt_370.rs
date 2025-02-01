use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_370: FileFormat = FileFormat {
    id: 542,
    puid: "x-fmt/370",
    name: "WordStar for MS-DOS Document",
    extensions: &["ws3", "ws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 378,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
