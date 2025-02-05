use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1754: FileFormat = FileFormat {
    id: 1_754,
    source_type: SourceType::Pronom,
    name: "WordPerfect",
    extensions: &["wp4", "wpd"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 736,
    }],
};
