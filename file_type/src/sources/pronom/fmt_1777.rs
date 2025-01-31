use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1777: FileFormat = FileFormat {
    id: 2_627,
    puid: "fmt/1777",
    name: "SIARD (Software-Independent Archiving of Relational Databases)",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_006,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
