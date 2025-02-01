use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1196: FileFormat = FileFormat {
    id: 2_006,
    puid: "fmt/1196",
    name: "SIARD (Software-Independent Archiving of Relational Databases)",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_627,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_800,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
