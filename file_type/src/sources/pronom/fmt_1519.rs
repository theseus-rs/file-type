use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1519: FileFormat = FileFormat {
    id: 2_343,
    puid: "fmt/1519",
    name: "Serif DrawPlus Drawing",
    extensions: &["dpp", "dpa", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_352,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
