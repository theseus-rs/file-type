use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1371: FileFormat = FileFormat {
    id: 2_189,
    puid: "fmt/1371",
    name: "OmniPage Pro Document",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_190,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
