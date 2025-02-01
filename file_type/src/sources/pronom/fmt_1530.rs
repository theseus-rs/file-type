use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1530: FileFormat = FileFormat {
    id: 2_354,
    puid: "fmt/1530",
    name: "Serif PagePlus Publication",
    extensions: &["ppp", "ppb", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_470,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
