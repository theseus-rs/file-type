use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1535: FileFormat = FileFormat {
    id: 2_359,
    puid: "fmt/1535",
    name: "Serif PagePlus Publication",
    extensions: &["ppp", "ppb", "ppx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_470,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
