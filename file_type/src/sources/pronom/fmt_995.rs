use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_995: FileFormat = FileFormat {
    id: 1_800,
    puid: "fmt/995",
    name: "SIARD (Software-Independent Archiving of Relational Databases)",
    extensions: &["siard"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_006,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
