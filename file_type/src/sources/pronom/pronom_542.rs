use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_542: FileFormat = FileFormat {
    id: 542,
    source_type: SourceType::Pronom,
    name: "WordStar for MS-DOS Document",
    extensions: &["ws3", "ws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 378,
    }],
};
