use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1522: FileFormat = FileFormat {
    id: 2_346,
    puid: "fmt/1522",
    name: "Serif DrawPlus Drawing",
    extensions: &["dpp", "dpa", "dpx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_352,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
