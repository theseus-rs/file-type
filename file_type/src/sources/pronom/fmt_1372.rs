use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1372: FileFormat = FileFormat {
    id: 2_190,
    puid: "fmt/1372",
    name: "OmniPage Document",
    extensions: &["opd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_189,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
