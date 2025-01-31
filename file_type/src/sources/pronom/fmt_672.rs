use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_672: FileFormat = FileFormat {
    id: 1_471,
    puid: "fmt/672",
    name: "Serif PagePlus Publication",
    extensions: &["ppp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 1_470,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
