use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1752: FileFormat = FileFormat {
    id: 2_599,
    puid: "fmt/1752",
    name: "OpenDocument Database Format",
    extensions: &["odb"],
    media_types: &["application/vnd.oasis.opendocument.base"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_231,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 778,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
