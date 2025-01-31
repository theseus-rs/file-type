use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_853: FileFormat = FileFormat {
    id: 1_654,
    puid: "fmt/853",
    name: "Serif DrawPlus Drawing",
    extensions: &["dpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_352,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_653,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
