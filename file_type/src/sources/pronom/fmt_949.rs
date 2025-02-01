use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_949: FileFormat = FileFormat {
    id: 1_754,
    puid: "fmt/949",
    name: "WordPerfect",
    extensions: &["wp4", "wpd"],
    media_types: &["application/vnd.wordperfect"],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 736,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
